use indexmap::{IndexMap, IndexSet};
use konfig_edit::error::{Error, Result};
use konfig_edit::value::Path;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
};
use std::ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};
use std::path::{Path as StdPath, PathBuf};
use std::rc::{Rc, Weak as RcWeak};
use std::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize,
};
use std::sync::{Arc, Mutex, RwLock, Weak as ArcWeak};
use std::time::{Duration, SystemTime};

pub trait WithDocs {
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()>;
}

macro_rules! impl_noop {
    ( $( $impl_desc:tt )* ) => {
        impl $( $impl_desc )* {
            #[inline]
            fn add_docs(
                &self,
                _path: &mut Path<'static>,
                _docs: &mut HashMap<Path<'static>, String>,
            ) -> Result<()> {
                Ok(())
            }
        }
    };
}

impl_noop!(<'i> WithDocs for fmt::Arguments<'i>);
impl_noop!(<T> WithDocs for PhantomData<T>);
impl_noop!(<T> WithDocs for [T; 0]);
impl_noop!(<Idx> WithDocs for Bound<Idx>);
impl_noop!(<Idx> WithDocs for Range<Idx>);
impl_noop!(<Idx> WithDocs for RangeFrom<Idx>);
impl_noop!(<Idx> WithDocs for RangeInclusive<Idx>);
impl_noop!(<Idx> WithDocs for RangeTo<Idx>);
impl_noop!(<Idx> WithDocs for RangeToInclusive<Idx>);
impl_noop!(<T> WithDocs for Reverse<T>);
impl_noop!(<T> WithDocs for Wrapping<T>);

macro_rules! impl_for_non_generic {
    ( $( $Ty:ty ),* ) => {
        $( impl_noop!(WithDocs for $Ty); )*
    };
}

impl_for_non_generic! {
    bool,
    char,
    f32,
    f64,
    i128,
    i16,
    i32,
    i64,
    i8,
    isize,
    str,
    u128,
    u16,
    u32,
    u64,
    u8,
    usize,
    String,
    (),
    AtomicBool,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicI8,
    AtomicIsize,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicU8,
    AtomicUsize,
    CStr,
    CString,
    OsStr,
    OsString,
    Duration,
    SystemTime,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    NonZeroI128,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI8,
    NonZeroIsize,
    NonZeroU128,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU8,
    NonZeroUsize,
    StdPath,
    PathBuf
}

macro_rules! impl_for_ref {
    ( $( $impl_desc:tt )* ) => {
        impl $( $impl_desc )* {
            #[inline]
            fn add_docs(
                &self,
                path: &mut Path<'static>,
                docs: &mut HashMap<Path<'static>, String>,
            ) -> Result<()> {
                (**self).add_docs(path, docs)
            }
        }
    };
}

impl_for_ref!(<'i, T> WithDocs for &'i T where T: WithDocs + ?Sized);
impl_for_ref!(<'i, T> WithDocs for &'i mut T where T: WithDocs + ?Sized);
impl_for_ref!(<T> WithDocs for Box<T> where T: WithDocs + ?Sized);
impl_for_ref!(<T> WithDocs for Rc<T> where T: WithDocs + ?Sized);
impl_for_ref!(<T> WithDocs for Arc<T> where T: WithDocs + ?Sized);
impl_for_ref!(<'i, T> WithDocs for Cow<'i, T> where T: WithDocs + ToOwned + ?Sized);

macro_rules! impl_for_weak {
    ( $( $Ty:ident ),* ) => {
        $(
            impl<T> WithDocs for $Ty<T>
            where
                T: WithDocs + ?Sized,
            {
                fn add_docs(
                    &self,
                    path: &mut Path<'static>,
                    docs: &mut HashMap<Path<'static>, String>,
                ) -> Result<()> {
                    self.upgrade().add_docs(path, docs)
                }
            }
        )*
    };
}

impl_for_weak!(RcWeak, ArcWeak);

macro_rules! impl_for_seq {
    ( $( $impl_desc:tt )* ) => {
        impl $( $impl_desc )* {
            fn add_docs(
                &self,
                path: &mut Path<'static>,
                docs: &mut HashMap<Path<'static>, String>,
            ) -> Result<()> {
                for (idx, elem) in self.iter().enumerate() {
                    path.push_sequence_index(idx);
                    docs.insert(path.clone(), format!("• `{idx}`"));
                    elem.add_docs(path, docs)?;
                    path.pop();
                }

                Ok(())
            }
        }
    };
}

impl_for_seq!(<T> WithDocs for [T] where T: WithDocs);
impl_for_seq!(<T> WithDocs for Vec<T> where T: WithDocs);
impl_for_seq!(<T> WithDocs for VecDeque<T> where T: WithDocs);
impl_for_seq!(<T> WithDocs for BinaryHeap<T> where T: WithDocs + Ord);
impl_for_seq!(<T> WithDocs for BTreeSet<T> where T: WithDocs + Ord);
impl_for_seq!(<T> WithDocs for HashSet<T> where T: WithDocs + Eq + Hash);
impl_for_seq!(<T> WithDocs for IndexSet<T> where T: WithDocs + Eq + Hash);
impl_for_seq!(<T> WithDocs for LinkedList<T> where T: WithDocs);

macro_rules! impl_for_array {
    ( $( $len:tt )* ) => {
        $( impl_for_seq!(<T> WithDocs for [T; $len] where T: WithDocs); )*
    };
}

impl_for_array! {
     1  2  3  4  5  6  7  8  9 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

macro_rules! impl_for_tuple {
    ( $( ( $( $idx:tt $Ty:ident )* ) ),* ) => {
        $(
            impl<$($Ty),*> WithDocs for ($($Ty,)*)
            where
                $( $Ty: WithDocs ),*
            {
                fn add_docs(
                    &self,
                    path: &mut Path<'static>,
                    docs: &mut HashMap<Path<'static>, String>,
                ) -> Result<()> {
                    $(
                        path.push_sequence_index($idx);
                        docs.insert(path.clone(), format!("• `{}`", $idx));
                        self.$idx.add_docs(path, docs)?;
                        path.pop();
                    )*

                    Ok(())
                }
            }
        )*
    };
}

impl_for_tuple! {
    (0 T0),
    (0 T0 1 T1),
    (0 T0 1 T1 2 T2),
    (0 T0 1 T1 2 T2 3 T3),
    (0 T0 1 T1 2 T2 3 T3 4 T4),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)

}

macro_rules! impl_for_map {
    ( $( $impl_desc:tt )* ) => {
        impl $( $impl_desc )* {
            fn add_docs(
                &self,
                path: &mut Path<'static>,
                docs: &mut HashMap<Path<'static>, String>,
            ) -> Result<()> {
                for (key, val) in self.iter() {
                    let key = key.to_string();
                    let doc = format!("• `{key}`");

                    path.push_map_key(key);
                    docs.insert(path.clone(), doc);
                    val.add_docs(path, docs)?;
                    path.pop();
                }

                Ok(())
            }
        }
    };
}

impl_for_map!(<K, V> WithDocs for HashMap<K, V> where K: Eq + Hash + fmt::Display, V: WithDocs);
impl_for_map!(<K, V> WithDocs for IndexMap<K, V> where K: Eq + Hash + fmt::Display, V: WithDocs);
impl_for_map!(<K, V> WithDocs for BTreeMap<K, V> where K: Ord + fmt::Display, V: WithDocs);

impl<T> WithDocs for Cell<T>
where
    T: WithDocs + Copy,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        self.get().add_docs(path, docs)
    }
}

impl<T> WithDocs for RefCell<T>
where
    T: WithDocs,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        match self.try_borrow() {
            Ok(v) => v.add_docs(path, docs),
            Err(_) => Err(Error::custom(
                "`RefCell` is mutably borrowed while trying to obtain docs",
            )),
        }
    }
}

impl<T> WithDocs for Mutex<T>
where
    T: WithDocs + ?Sized,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        match self.lock() {
            Ok(v) => v.add_docs(path, docs),
            Err(_) => Err(Error::custom(
                "`Mutex` is poisoned while trying to obtain docs",
            )),
        }
    }
}

impl<T> WithDocs for RwLock<T>
where
    T: WithDocs + ?Sized,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        match self.read() {
            Ok(v) => v.add_docs(path, docs),
            Err(_) => Err(Error::custom(
                "`RwLock` is poisoned while trying to obtain docs",
            )),
        }
    }
}

impl<T> WithDocs for Option<T>
where
    T: WithDocs,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        if let Some(v) = self {
            v.add_docs(path, docs)?;
        }

        Ok(())
    }
}

impl<T, E> WithDocs for std::result::Result<T, E>
where
    T: WithDocs,
    E: WithDocs,
{
    fn add_docs(
        &self,
        path: &mut Path<'static>,
        docs: &mut HashMap<Path<'static>, String>,
    ) -> Result<()> {
        match self {
            Ok(v) => v.add_docs(path, docs),
            Err(e) => e.add_docs(path, docs),
        }
    }
}
