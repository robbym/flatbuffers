//extern crate num_traits;
//use std::convert::AsMut;

//pub trait ToLittleEndian {
//    fn to_le(self) -> Self {
//        self.to_le()
//    }
//}

//impl ToLittleEndian for i16 {}


const FLATBUFFERS_MAX_BUFFER_SIZE: usize = ((1u64 << 32) - 1) as usize;

use std::marker::PhantomData;
//use std::iter::FromIterator;

pub type VectorOffset = ();
pub type StringOffset = ();
pub type ByteStringOffset = ();
pub trait ElementScalar : Sized {
    fn to_le(self) -> Self;
}
//impl ElementScalar for bool { fn to_le(self) -> bool { u8::to_le(self as u8) as bool } }
impl ElementScalar for u8 { fn to_le(self) -> u8 { u8::to_le(self) } }
impl ElementScalar for i8 { fn to_le(self) -> i8 { i8::to_le(self) } }
impl ElementScalar for u16 { fn to_le(self) -> u16 { u16::to_le(self) } }
impl ElementScalar for i16 { fn to_le(self) -> i16 { i16::to_le(self) } }
impl ElementScalar for u32 { fn to_le(self) -> u32 { u32::to_le(self) } }
impl ElementScalar for i32 { fn to_le(self) -> i32 { i32::to_le(self) } }
impl ElementScalar for u64 { fn to_le(self) -> u64 { u64::to_le(self) } }
impl ElementScalar for i64 { fn to_le(self) -> i64 { i64::to_le(self) } }

pub const VTABLE_METADATA_FIELDS: usize = 2;
pub const SIZE_U8: usize = 1;
pub const SIZE_I8: usize = 1;
pub const SIZE_U16: usize = 2;
pub const SIZE_I16: usize = 2;
pub const SIZE_U32: usize = 4;
pub const SIZE_I32: usize = 4;
pub const SIZE_U64: usize = 8;
pub const SIZE_I64: usize = 8;
pub const SIZE_UOFFSET: usize = SIZE_U32;
pub const SIZE_SOFFSET: usize = SIZE_I32;
pub const SIZE_VOFFSET: usize = SIZE_I16;

#[inline]
pub fn padding_bytes(buf_size: usize, scalar_size: usize) -> usize {
  ((!buf_size) + 1) & (scalar_size - 1)
}
pub fn field_index_to_offset(field_id: VOffsetT) -> VOffsetT {
    // Should correspond to what end_table() below builds up.
    let fixed_fields = 2;  // Vtable size and Object Size.
    ((field_id + fixed_fields) * (SIZE_VOFFSET as VOffsetT)) as VOffsetT
}
pub fn emplace_scalar<T>(s: &mut [u8], x: T) {
    let sz = std::mem::size_of::<T>();
    let data = unsafe {
        std::slice::from_raw_parts((&x as *const T) as *const u8, sz)
    };

    s[..sz].copy_from_slice(data);
}

pub trait Table {}
pub struct Verifier {}
impl Verifier {
    pub fn new() -> Self {
        Verifier{}
    }
    pub fn verify<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_buffer<T>(&mut self, _: &'static str) -> bool {
        false
    }
    pub fn verify_vector_of_strings<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_vector_of_tables<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_table<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn end_table(&mut self) -> bool {
        false
    }
}
pub struct TypeTable {}
pub struct FlatBufferBuilder<'fbb> {
    pub owned_buf: Vec<u8>,
    pub cur_idx: usize,

    vtable: Vec<UOffsetT>,
    vtables: Vec<UOffsetT>,

    nested: bool,
    finished: bool,

    min_align: usize,
    table_end: UOffsetT,
    max_voffset: VOffsetT,
    _phantom: PhantomData<&'fbb ()>,
}
//impl<T> AsMut<T> for FlatBufferBuilder {
//    fn as_mut(&mut self) -> &mut FlatBufferBuilder {
//        self
//    }
//}
impl<'fbb> FlatBufferBuilder<'fbb> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }
    pub fn new_with_capacity(size: usize) -> Self {
        FlatBufferBuilder{
            owned_buf: vec![0u8; size],
            vtable: Vec::new(),
            vtables: Vec::new(),

            cur_idx: size,

            nested: false,
            finished: false,

            min_align: 0,
            table_end: 0,

            max_voffset: 0,
            _phantom: PhantomData,
        }
    }
    pub fn start_table(&mut self, num_fields: VOffsetT) -> UOffsetT {
        self.assert_not_nested();
        self.nested = true;

        self.vtable.clear();
        self.vtable.truncate(num_fields as usize);

        self.min_align = 1;
        self.table_end = self.rev_cur_idx();

        self.get_size() as UOffsetT
    }
    pub fn get_buf_slice(&self) -> &[u8] {
        &self.owned_buf[..]
    }
    pub fn get_active_buf_slice(&self) -> &[u8] {
        &self.owned_buf[self.cur_idx..]
    }
    pub fn get_mut_active_buf_slice(&mut self) -> &mut [u8] {
        &mut self.owned_buf[self.cur_idx..]
    }
    pub fn reallocate(&mut self, _: usize) {
        unimplemented!()
    }

    pub fn grow_owned_buf(&mut self) -> usize {
        let starting_active_size = self.get_active_buf_slice().len();
        assert!(self.owned_buf.len() * 2 <= FLATBUFFERS_MAX_BUFFER_SIZE,
		        "cannot grow buffer beyond 2 gigabytes");

        let old_len = self.owned_buf.len();
        let new_len = std::cmp::max(1, self.owned_buf.len() * 2);
        let diff = new_len - old_len;

        self.owned_buf.resize(new_len, 0);
        self.cur_idx += diff;
        let ending_active_size = self.get_active_buf_slice().len();
        assert_eq!(starting_active_size, ending_active_size);
        if self.owned_buf.len() == 1 {
            return 1;
        }

		// calculate the midpoint, and safely copy the old end data to the new
		// end position:
		let middle = new_len / 2;
		{
			let (left, right) = &mut self.owned_buf[..].split_at_mut(middle);
            //println!("foo {}, {:?}, {:?}", middle, &left[..], &right[..]);
			right.copy_from_slice(left);
		}
        // then, zero out the old end data (just to be safe):
        // should be vectorized by the compiler--rust has no stdlib memset.
        for x in &mut self.owned_buf[..middle] {
            *x = 0;
        }

        let ending_active_size = self.get_active_buf_slice().len();
        assert_eq!(starting_active_size, ending_active_size);

        new_len
	}
    //pub fn as_mut(&mut self) -> &mut Self {
    //    self
    //}
    //
    pub fn assert_nested(&self) {
        assert!(self.nested);
        // vtable could be empty (e.g. for all-default values) or in a vector
    }
    pub fn assert_not_nested(&self) {
        assert!(!self.nested);
        assert_eq!(self.vtable.len(), 0);
    }
    pub fn start_vector(&mut self, elemsize: usize, num_elems: usize) -> UOffsetT {
        self.assert_not_nested();
        self.nested = true;
        self.pre_align(num_elems*elemsize, SIZE_UOFFSET);
        self.pre_align(num_elems*elemsize, elemsize); // Just in case elemsize > uoffset_t.
        self.rev_cur_idx()
    }
    // Offset relative to the end of the buffer.
    pub fn rev_cur_idx(&self) -> UOffsetT {
        (self.owned_buf.len() - self.cur_idx) as UOffsetT
    }
    pub fn end_vector(&mut self, len: usize) -> UOffsetT {
      self.assert_nested();
      self.push_element_scalar(len as UOffsetT);
      self.nested = false;
      self.rev_cur_idx()
  }
    pub fn emplace_scalar_in_active_buf<T>(&mut self, at: usize, x: T) {
        let buf = &mut self.get_mut_active_buf_slice();
        emplace_scalar(&mut buf[at..], x)
    }
    pub fn pre_align(&mut self, n: usize, alignment: usize) {
        self.track_min_align(alignment);
        let s = self.get_size() as usize;
        self.fill(padding_bytes(s + n, alignment));
    }
    pub fn get_size(&self) -> usize {
        self.owned_buf.len() - self.cur_idx
    }
    pub fn fill(&mut self, zero_pad_bytes: usize) {
        self.make_space(zero_pad_bytes);
        let buf = &mut self.get_mut_active_buf_slice();
        for i in 0..zero_pad_bytes {
            buf[i] = 0;
        }
    }
    pub fn track_min_align(&mut self, alignment: usize) {
        self.min_align = std::cmp::max(self.min_align, alignment);
    }
    pub fn add_element<T>(&mut self, _: isize, _: T, _: T) -> T {
        unimplemented!()
    }
    pub fn add_offset<T>(&mut self, _: isize, _: LabeledUOffsetT<T>) -> usize {
        unimplemented!()
    }
    pub fn add_struct<T>(&mut self, _: isize, _: T) {
        unimplemented!()
    }
    // utf-8 string creation
    pub fn create_string(&mut self, s: &str) -> LabeledUOffsetT<StringOffset> {
        self.create_byte_string(s.as_bytes())
    }
    pub fn create_byte_string<'a>(&mut self, data: &[u8]) -> LabeledUOffsetT<ByteStringOffset> {
        self.assert_not_nested();
        self.pre_align(data.len() + 1, SIZE_UOFFSET);  // Always 0-terminated.
        self.fill(1);
        self.push_bytes(data);
        self.push_element_scalar(data.len() as UOffsetT);
        LabeledUOffsetT::new(self.get_size() as u32)
    }
    pub fn create_shared_string<'a>(&mut self, _: &'a str) -> LabeledUOffsetT<StringOffset> {
        LabeledUOffsetT::new(0)
    }
    //pub fn create_vector_of_strings<'a, 'b, T: 'b>(&'a mut self, _: &'b [T]) -> Offset<&'b [T]> {
    //pub fn create_vector_of_strings<'a>(&mut self, _: &'a [&'a str]) -> LabeledUOffsetT<VectorOffset<StringOffset>> {
    pub fn create_vector_of_strings<'a>(&mut self, _: &'a [&'a str]) -> LabeledUOffsetT<VectorOffset> {
        LabeledUOffsetT::new(0)
    }
    //pub fn create_vector<T, V: FromIterator<T>>(&mut self, _: V) -> Offset<Vector<T>> {
    pub fn create_vector<'a, T: 'a>(&'a mut self, _: &'a [T]) -> LabeledUOffsetT<&'fbb [T]> {
        LabeledUOffsetT::new(0)
    }
//  //pub fn create_vector_from_fn<'a: 'fbb, 'b, T: 'b, F: FnMut(usize, &mut Self) -> T>(&'fbb mut self, _len: usize, _f: F) -> Offset<&'b [T]> {
    pub fn create_vector_from_fn<F, T>(&mut self, _len: usize, _f: F) -> LabeledUOffsetT<&'fbb [T]>
        where F: FnMut(usize, &mut Self) -> T {
        LabeledUOffsetT::new(0)
    }
//  pub fn create_vector_of_structs<'a, T: 'a>(&'fbb mut self, _: &'a [T]) -> Offset<&'a [T]> {
//      LabeledUOffsetT::new(0)
//  }
//  // TODO probably should not be returning [&T]
    pub fn create_vector_of_sorted_structs<'a, T>(&mut self, _: &'a mut [T]) -> LabeledUOffsetT<&'fbb [&'fbb T]> {
        LabeledUOffsetT::new(0)
    }
    pub fn create_vector_of_structs_from_fn<T, F>(&mut self, _len: usize, _f: F) -> LabeledUOffsetT<&'fbb [&'fbb T]>
        where F: FnMut(usize, &mut T) {
        LabeledUOffsetT::new(0)
    }
    pub fn create_vector_of_sorted_tables<'a, T>(&mut self, _: &'a mut [T]) -> LabeledUOffsetT<&'fbb [T]> {
        LabeledUOffsetT::new(0)
    }
    pub fn dump_buf(&self, label: &str) {
        println!("dump_buf {}: {}/{}: {:?}", label, self.get_size(), self.owned_buf.len(), self.get_active_buf_slice());
    }
    //pub fn end_table3(&mut self, start: UOffsetT) -> UOffsetT {
    //    self.assert_nested();
    //    self.push_element_scalar::<SOffsetT>(0);
    //    let object_offset = b.get_size();
    //}
    pub fn end_table(&mut self, _: UOffsetT) -> UOffsetT {
        self.assert_nested();
        let n = self.write_vtable();
        self.nested = false;
        n
    }
    pub fn write_vtable(&mut self) -> UOffsetT {
        self.push_element_scalar::<SOffsetT>(0);
        let table_offset = self.rev_cur_idx();

        // empty vtable for now
        let table_size = table_offset - self.table_end;
        self.push_element_scalar::<VOffsetT>(table_size as VOffsetT);
        let vtable_size = (0 + VTABLE_METADATA_FIELDS) * SIZE_VOFFSET;
        self.push_element_scalar::<VOffsetT>(vtable_size as VOffsetT);
        let table_start = self.owned_buf.len() as SOffsetT - table_offset as SOffsetT;
        {
            let n = self.rev_cur_idx() as SOffsetT - table_offset as SOffsetT;
            emplace_scalar::<SOffsetT>(&mut self.owned_buf[table_start as usize..],n);
        }


        0
    }
    pub fn end_table_old(&mut self, start: UOffsetT) -> UOffsetT {
        self.assert_nested();

        let vtableoffsetloc = self.push_element_scalar::<SOffsetT>(0);
        self.dump_buf(&format!("pushed empty vtableoffsetloc {}", vtableoffsetloc));
        self.max_voffset = std::cmp::max(self.max_voffset + SIZE_VOFFSET as VOffsetT,
                                         field_index_to_offset(0));
        let to_fill = self.max_voffset as usize;
        self.fill(to_fill);
        self.dump_buf(&format!("filled {}", to_fill));

        let table_object_size = vtableoffsetloc - start;
        assert!(table_object_size < 0x10000);  // Vtable use 16bit offsets.

        {
            let n = table_object_size as VOffsetT;
            emplace_scalar::<VOffsetT>(&mut self.get_mut_active_buf_slice()[SIZE_VOFFSET..], n);
            self.dump_buf(&format!("after placing table_object_size {}", n));
        }
        {
            let n = self.max_voffset as VOffsetT;
            emplace_scalar::<VOffsetT>(&mut self.get_mut_active_buf_slice(), n);
            self.dump_buf(&format!("after placing max_voffset {}", n));
        }

        let vt_use = self.get_size();
        println!("vt_use at start: {}", vt_use);
        // TODO write vtable

        {
            let n = (vt_use as SOffsetT) - (vtableoffsetloc as SOffsetT);
            //let i = self.get_size() - vtableoffsetloc as usize;
            let buf = &mut self.get_mut_active_buf_slice();
            //let i = buf.len() - vtableoffsetloc as usize;
            let i = vtableoffsetloc as usize;
            println!("writing vt_use... {} at {} -- {:?}", n, i, buf);
            emplace_scalar::<SOffsetT>(&mut buf[i..], n);
        }

        self.nested = false;

        vtableoffsetloc as UOffsetT
    }

    pub fn required<T>(&self, _: &LabeledUOffsetT<T>, _: isize) -> bool {
        unimplemented!()
    }
    pub fn finish<T>(&mut self, _root: LabeledUOffsetT<T>)  {
    }
    pub fn finish_with_identifier<'a, T>(&'a mut self, _root: LabeledUOffsetT<T>, _name: &'static str) {
    }

    pub fn release_buffer_pointer(&mut self) -> DetachedBuffer  {
       //self.Finished();
       // self.buf_.release();
       DetachedBuffer{}
    }

    pub fn push_element_bool(&mut self, b: bool) -> UOffsetT {
        self.push_element_scalar(b as u8)
    }
    fn align(&mut self, elem_size: usize) {
        let delta = self.cur_idx % elem_size;
        self.cur_idx -= delta;
    }
    pub fn push_element_scalar<T: ElementScalar>(&mut self, t: T) -> UOffsetT {
        let t = t.to_le(); // convert to little-endian
        self.align(std::mem::size_of::<T>());
        self.push(t); // TODO: push_small
        self.cur_idx as UOffsetT
    }
    pub fn push_bytes(&mut self, x: &[u8]) -> UOffsetT {
        let n = self.make_space(x.len());//std::mem::size_of::<T>());
        {
            let buf = &mut self.get_mut_active_buf_slice();
            buf[..x.len()].copy_from_slice(x);
        }

        self.cur_idx as UOffsetT
    }

    pub fn push<T: Sized>(&mut self, x: T) {
        println!("start of push: {}", self.cur_idx);
        let s = std::mem::size_of::<T>();
        println!("make space {}", s);
        let n = self.make_space(s);
        {
            let buf = &mut self.get_mut_active_buf_slice();
            emplace_scalar(&mut buf[..s], x);
        }
        println!("after push: {} {:?} {:?}", self.cur_idx, self.get_active_buf_slice(), &self.owned_buf[..]);
    }

    pub fn release(&mut self) {
        //DetachedBuffer fb(allocator_, own_allocator_, buf_, reserved_, cur_,
        //                  size());
        //allocator_ = nullptr;
        //own_allocator_ = false;
        //buf_ = nullptr;
        //clear();
        //return fb;
    }

    pub fn make_space(&mut self, want: usize) -> usize {
        self.ensure_space(want);
        self.cur_idx -= want;
        self.cur_idx
    }
    pub fn ensure_space(&mut self, want: usize) -> usize {
        assert!(want <= FLATBUFFERS_MAX_BUFFER_SIZE,
		        "cannot grow buffer beyond 2 gigabytes");
        while self.cur_idx < want {
            println!("growing: {} < {}: {:?}", self.cur_idx, want, self.get_active_buf_slice());
            self.grow_owned_buf();
            println!("grew to: {}, {}, {:?}", self.cur_idx, self.owned_buf.len(), self.get_active_buf_slice());
        }
        want
    }
}
pub trait UOffsetTTrait {}
pub trait OffsetTTrait {}
pub trait VOffsetTTrait {}
pub type UOffsetT = u32;
pub type SOffsetT = i32;
pub type VOffsetT = i16;

//pub type String<'a> = &'a str;
pub type Void<'a> = &'a [u8];
pub struct Vector<T>  {
    phantom: PhantomData<T>,
}

//pub struct LabeledUOffsetT<T> (usize, PhantomData<T>);
pub struct VectorLabeledUOffsetT<T> (usize, PhantomData<T>);
//pub struct ULabeledUOffsetT<T> (u32, PhantomData<T>);

//impl<T> LabeledUOffsetT<T> {
//    pub fn new(o: usize) -> Self {
//        Offset(o, PhantomData)
//    }
//    pub fn union(&self) -> Offset<Void> {
//        LabeledUOffsetT::new(self.0)
//    }
//}
//impl<T> ULabeledUOffsetT<T> {
//    pub fn new(o: u32) -> Self {
//        UOffset(o, PhantomData)
//    }
//}
pub struct LabeledUOffsetT<T> (UOffsetT, PhantomData<T>);
impl<T> Copy for LabeledUOffsetT<T> { } // TODO: why does deriving Copy cause ownership errors?
impl<T> Clone for LabeledUOffsetT<T> {
    fn clone(&self) -> LabeledUOffsetT<T> {
        LabeledUOffsetT::new(self.0.clone())
    }
}

impl<T> LabeledUOffsetT<T> {
    pub fn new(o: UOffsetT) -> Self {
        LabeledUOffsetT(o, PhantomData)
    }
    pub fn union(&self) -> LabeledUOffsetT<Void> {
        LabeledUOffsetT::new(self.0)
    }
}

//impl<T> From<usize> for ULabeledUOffsetT<T> { fn from(n: usize) -> Self { ULabeledUOffsetT::new(n) } }
//impl<T> From<isize> for LabeledUOffsetT<T> { fn from(n: isize) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<u8> for LabeledUOffsetT<T>  { fn from(n: u8)  -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<u16> for LabeledUOffsetT<T> { fn from(n: u16) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<u32> for LabeledUOffsetT<T> { fn from(n: u32) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<u64> for LabeledUOffsetT<T> { fn from(n: u64) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<i8> for LabeledUOffsetT<T>  { fn from(n: i8)  -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<i16> for LabeledUOffsetT<T> { fn from(n: i16) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<i32> for LabeledUOffsetT<T> { fn from(n: i32) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<i64> for LabeledUOffsetT<T> { fn from(n: i64) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<usize> for LabeledUOffsetT<T> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }
//impl<T> From<isize> for LabeledUOffsetT<T> { fn from(n: isize) -> Self { LabeledUOffsetT::new(n) } }
//impl From<usize> for Offset<u16> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }
//impl From<usize> for Offset<u32> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }
//impl From<usize> for Offset<u64> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }
//impl From<usize> for Offset<f32> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }
//impl From<usize> for Offset<f64> { fn from(n: usize) -> Self { LabeledUOffsetT::new(n) } }

pub fn verify_table_start(_: &Verifier) -> bool {
    false
}
//pub fn endian_scalar<T: num_traits::int::PrimInt>(x: T) -> T {
//    x.to_le()
//}
pub fn endian_scalar<T>(x: T) -> T {
    x
    //x.to_le()
}
pub fn write_scalar<S, T>(_: S, _: T) -> ! {
    unimplemented!()
}
pub fn set_field<T>(_: isize, _: T, _: T) -> bool {
    unimplemented!()
}
pub fn verify_field<T>(_: &Verifier, _: isize) -> bool {
    false
}
pub fn verify_offset(_: &Verifier, _: isize) -> ! {
    unimplemented!()
}
pub fn verify_offset_required(_: &Verifier, _: isize) -> ! {
    unimplemented!()
}
pub fn get_root<T>(_: &[u8]) -> T {
    unimplemented!()
}
pub fn get_mutable_root<T>(_: &[u8]) -> T {
    unimplemented!()
}
pub fn get_struct<T>(_: isize) -> T {
    unimplemented!()
}
pub fn get_struct_mut<T>(_: isize) -> T {
    unimplemented!()
}
pub fn get_field<T>(_: isize, _: T) -> T {
    unimplemented!()
}
//pub fn get_field<T: num_traits::Num>(_: isize, _: T) -> T {
//    unimplemented!()
//}
//pub fn get_field_mut<T: num_traits::Num>(_: isize, _: T) -> T {
//    unimplemented!()
//}
pub fn get_pointer<'a, T: 'a>(_: isize) -> &'a T {
    unimplemented!()
}
pub fn get_pointer_mut<'a, T: 'a>(_: isize) -> &'a mut T {
    unimplemented!()
}
pub fn buffer_has_identifier<S, T>(_: S, _: T) -> bool {
    false
}
pub struct DetachedBuffer {}
pub mod flexbuffers {
    pub struct Reference {}
pub fn get_root<T>(_: &[u8], _: isize) -> T {
    unimplemented!()
}

}
