use core::mem::MaybeUninit;

use crate::drivers::{BlockDriver, BlockDriverWrapper};
use crate::drivers::{DeviceType, DRIVERS};
use rcore_fs::{dev::Device, vfs};

pub mod inode_ext;

use alloc::sync::Arc;
use inode_ext::INodeExt;
use lazy_static::*;
use rcore_fs::{dev::block_cache::BlockCache, vfs::INode};
use rcore_fs_mountfs::MountFS;
use rcore_fs_sfs::{AsBuf, BlockId, SimpleFileSystem, SuperBlock, BLKN_SUPER};

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        let device = {
            let blc_dvice = crate::drivers::BLK_DRIVERS
                .read().iter()
                .next().expect("Block device not found")
                .clone();
            let driver = BlockDriverWrapper(
                blc_dvice
            );
            // debug!("{:?}",crate::drivers::BLK_DRIVERS.read();
            debug!("{}",crate::drivers::BLK_DRIVERS.read().len());
            // enable block cache
            // Arc::new(BlockCache::new(driver, 0x100))
            Arc::new(driver)
        };

        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        let rootfs = MountFS::new(sfs);
        let root = rootfs.root_inode();
        root
    };
}

pub fn init() {
    ROOT_INODE.ls();
    debug!("Fs init!");
}
