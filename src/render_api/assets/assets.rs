use std::{marker::PhantomData, sync::{mpsc::{channel, sync_channel, Receiver, Sender}, Arc}};

use bevy_ecs::system::Resource;

// TODO: Assets<T> won't ever shrink, need to implement a way to clean up.
// It shouldn't be a problem if the user doesn't create a ton of Assets
// Although if you leave something running it might start a leak
// It could use a garbage collector sorta thing, but I don't feel like that's the best way to fix it
// I'll think about it.

#[derive(Debug, Copy, Clone)]
pub struct AssetHandle(u32, usize); // Magic, Inde

enum AssetEventType
{
    Drop,
    Clone
}

// Thanks Bevy for the idea
struct AssetEvent
{
    magic : u32,
    index : usize,
    ty :  AssetEventType
}

#[derive(Resource)]
pub struct Assets<T>
{
    assets : Vec<Option<(u32, T, u32)>>,
    magic : u32,
}

impl<T> Assets<T>
{
    pub fn new() -> Self
    {
        Self
        {
            assets: vec![],
            magic : 0,
        }
    }

    pub fn add_asset(&mut self, v : T) -> AssetHandle
    {
        self.magic = self.magic.overflowing_add(1).0;
        let index = {
            self.assets.iter()
                .enumerate()
                .find_map(|v| if v.1.is_none() {Some(v.0)} else {None})
                .unwrap_or_else(||
                {
                    self.assets.push(None);
                    self.assets.len() - 1
                })
        };
        self.assets[index] = Some((self.magic, v, 1));
        AssetHandle(self.magic, index)
    }

    pub fn remove_asset(&mut self, handle : &AssetHandle) -> Result<(), ()>
    {
        let asset = self.get_asset_raw_mut(handle);

        if asset.is_none()
        {
            return Err(())
        }

        *asset.unwrap() = None;

        Ok(())
    }

    fn remove_asset_unchecked(&mut self, index : usize) -> Result<(), ()>
    {
        let asset = &mut self.assets[index];

        *asset = None;

        Ok(())
    }


    /// Return type is wild
    fn get_asset_raw_mut(&mut self, handle : &AssetHandle) -> Option<&mut Option<(u32, T, u32)>>
    {
        let magic = handle.0;
        let index = handle.1;

        if let Some(asset_raw) = self.assets.get_mut(index)
        {
            if let Some((asset_magic, asset, references)) = asset_raw
            {
                if magic == *asset_magic
                {
                    return Some(asset_raw)
                }
            }
        }

        None
    }

    pub fn get_asset(&self, handle : &AssetHandle) -> Option<&T>
    {
        let magic = handle.0;
        let index = handle.1;

        if let Some(asset) = self.assets.get(index)
        {
            if let Some((asset_magic, asset, _)) = asset
            {
                if magic == *asset_magic
                {
                    return Some(asset)
                }
            }
        }

        None
    }

    pub fn get_asset_mut(&mut self, handle : &AssetHandle) -> Option<&mut T>
    {
        let magic = handle.0;
        let index = handle.1;

        if let Some(asset) = self.assets.get_mut(index)
        {
            if let Some((asset_magic, asset, _)) = asset
            {
                if magic == *asset_magic
                {
                    return Some(asset)
                }
            }
        }
        
        None
    }

    // pub fn cleanup(&mut self)
    // {
    //     while let Ok(message) = self.reciever.try_recv()
    //     {
    //         let ty = message.ty;
    //         let magic = message.magic;
    //         let index = message.index;

    //         match ty
    //         {
    //             AssetEventType::Drop => 
    //             {
    //                 let asset : &Option<_> = &self.assets[index];
    //                 if asset.is_none()
    //                 {
    //                     continue;
    //                 }
    //                 self.assets[index].take();
    //             },
    //             AssetEventType::Clone => 
    //             {
    //                 let asset = &mut self.assets[index];
    //                 if asset.is_none()
    //                 {
    //                     continue;
    //                 }
    //                 if let Some(asset) = asset
    //                 {
    //                     asset.2 += 1;
    //                 }
    //             },
    //         }
    //     }
    // }
}