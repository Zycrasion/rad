use std::marker::PhantomData;

// TODO: Assets<T> won't ever shrink, need to implement a way to clean up.
// It shouldn't be a problem if the user doesn't create a ton of Assets
// Although if you leave something running it might start a leak
// It could use a garbage collector sorta thing, but I don't feel like that's the best way to fix it
// I'll think about it.

pub struct AssetHandle(u32, usize); // Index, Magic

pub struct Assets<T>
{
    assets : Vec<Option<(u32, T)>>,
    magic : u32
}

impl<T> Assets<T>
{
    pub fn new() -> Self
    {
        Self
        {
            assets: vec![],
            magic : 0
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
        self.assets[index] = Some((self.magic, v));
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

    /// Return type is wild
    fn get_asset_raw_mut(&mut self, handle : &AssetHandle) -> Option<&mut Option<(u32, T)>>
    {
        let magic = handle.0;
        let index = handle.1;

        if let Some(asset_raw) = self.assets.get_mut(index)
        {
            if let Some((asset_magic, asset)) = asset_raw
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
            if let Some((asset_magic, asset)) = asset
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
            if let Some((asset_magic, asset)) = asset
            {
                if magic == *asset_magic
                {
                    return Some(asset)
                }
            }
        }
        
        None
    }
}