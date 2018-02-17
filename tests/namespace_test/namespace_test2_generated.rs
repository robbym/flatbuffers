// automatically generated by the FlatBuffers compiler, do not modify


extern crate flatbuffers;

#include "namespace_test1_generated.rs"

pub mod NamespaceA {

impl flatbuffers::Table for TableInFirstNS {}
impl TableInFirstNS /* private flatbuffers::Table */ {
    const VT_FOO_TABLE: isize = 4;
    const VT_FOO_ENUM: isize = 6;
    const VT_FOO_STRUCT: isize = 8;

  fn foo_table() -> &NamespaceA::NamespaceB::TableInNestedNS  {
    self.GetPointer::<&NamespaceA::NamespaceB::TableInNestedNS>(VT_FOO_TABLE)
  }
  fn mutable_foo_table(&mut self) -> &mut NamespaceA::NamespaceB::TableInNestedNS  {
    /* TODO: are there non-reference choices here? */
    &mut GetPointer::<&mut NamespaceA::NamespaceB::TableInNestedNS >(VT_FOO_TABLE)
  }
  fn foo_enum() -> NamespaceA::NamespaceB::EnumInNestedNS  {
    self.GetField::<i8>(VT_FOO_ENUM, 0) as NamespaceA::NamespaceB::EnumInNestedNS
  }
  fn mutate_foo_enum(foo_enum_: NamespaceA::NamespaceB::EnumInNestedNS) -> bool {
    SetField::<i8>(VT_FOO_ENUM, _foo_enum as i8, 0)
  }
  fn foo_struct() -> &NamespaceA::NamespaceB::StructInNestedNS  {
    self.GetStruct::<&NamespaceA::NamespaceB::StructInNestedNS>(VT_FOO_STRUCT)
  }
  fn mutable_foo_struct(&mut self) -> &mut NamespaceA::NamespaceB::StructInNestedNS  {
    /* TODO: are there non-reference choices here? */
    &mut GetStruct::<&mut NamespaceA::NamespaceB::StructInNestedNS >(VT_FOO_STRUCT)
  }
  fn Verify(verifier: &flatbuffers::Verifier) -> bool {
    return VerifyTableStart(verifier) &&
           VerifyOffset(verifier, VT_FOO_TABLE) &&
           verifier.VerifyTable(foo_table()) &&
           VerifyField::<i8>(verifier, VT_FOO_ENUM) &&
           VerifyField::<NamespaceA::NamespaceB::StructInNestedNS>(verifier, VT_FOO_STRUCT) &&
           verifier.EndTable();
  }
}

struct TableInFirstNSBuilder {
  fbb_: &flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::uoffset_t,
}
impl TableInFirstNSBuilder {
  fn add_foo_table(foo_table: flatbuffers::Offset<NamespaceA::NamespaceB::TableInNestedNS> ) {
    fbb_.AddOffset(TableInFirstNS::VT_FOO_TABLE, foo_table);
  }
  fn add_foo_enum(foo_enum: NamespaceA::NamespaceB::EnumInNestedNS ) {
    fbb_.AddElement::<i8>(TableInFirstNS::VT_FOO_ENUM, foo_enum as i8, 0);
  }
  fn add_foo_struct(foo_struct: &NamespaceA::NamespaceB::StructInNestedNS) {
    fbb_.AddStruct(TableInFirstNS::VT_FOO_STRUCT, foo_struct);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> TableInFirstNSBuilder {
    TableInFirstNSBuilder {
      fbb_: _fbb,
      start_: _fbb.StartTable(),
    }
  }
  // TableInFirstNSBuilder &operator=(const TableInFirstNSBuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<TableInFirstNS> {
    let end = self.fbb_.EndTable(self.start_);
    let o = end as flatbuffers::Offset<TableInFirstNS>;
    o
  }
}

#[inline]
fn CreateTableInFirstNS(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    foo_table: flatbuffers::Offset<NamespaceA::NamespaceB::TableInNestedNS>  /* = 0 */,
    foo_enum: NamespaceA::NamespaceB::EnumInNestedNS  /* = NamespaceA::NamespaceB::EnumInNestedNS_A */,
    foo_struct: &NamespaceA::NamespaceB::StructInNestedNS /* = 0 */) -> flatbuffers::Offset<TableInFirstNS> {
  let mut builder = TableInFirstNSBuilder::new(_fbb);
  builder_.add_foo_struct(foo_struct);
  builder_.add_foo_table(foo_table);
  builder_.add_foo_enum(foo_enum);
  builder_.Finish()
}

}  // pub mod NamespaceA

pub mod NamespaceC {

impl flatbuffers::Table for TableInC {}
impl TableInC /* private flatbuffers::Table */ {
    const VT_REFER_TO_A1: isize = 4;
    const VT_REFER_TO_A2: isize = 6;

  fn refer_to_a1() -> &NamespaceA::TableInFirstNS  {
    self.GetPointer::<&NamespaceA::TableInFirstNS>(VT_REFER_TO_A1)
  }
  fn mutable_refer_to_a1(&mut self) -> &mut NamespaceA::TableInFirstNS  {
    /* TODO: are there non-reference choices here? */
    &mut GetPointer::<&mut NamespaceA::TableInFirstNS >(VT_REFER_TO_A1)
  }
  fn refer_to_a2() -> &NamespaceA::SecondTableInA  {
    self.GetPointer::<&NamespaceA::SecondTableInA>(VT_REFER_TO_A2)
  }
  fn mutable_refer_to_a2(&mut self) -> &mut NamespaceA::SecondTableInA  {
    /* TODO: are there non-reference choices here? */
    &mut GetPointer::<&mut NamespaceA::SecondTableInA >(VT_REFER_TO_A2)
  }
  fn Verify(verifier: &flatbuffers::Verifier) -> bool {
    return VerifyTableStart(verifier) &&
           VerifyOffset(verifier, VT_REFER_TO_A1) &&
           verifier.VerifyTable(refer_to_a1()) &&
           VerifyOffset(verifier, VT_REFER_TO_A2) &&
           verifier.VerifyTable(refer_to_a2()) &&
           verifier.EndTable();
  }
}

struct TableInCBuilder {
  fbb_: &flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::uoffset_t,
}
impl TableInCBuilder {
  fn add_refer_to_a1(refer_to_a1: flatbuffers::Offset<NamespaceA::TableInFirstNS> ) {
    fbb_.AddOffset(TableInC::VT_REFER_TO_A1, refer_to_a1);
  }
  fn add_refer_to_a2(refer_to_a2: flatbuffers::Offset<NamespaceA::SecondTableInA> ) {
    fbb_.AddOffset(TableInC::VT_REFER_TO_A2, refer_to_a2);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> TableInCBuilder {
    TableInCBuilder {
      fbb_: _fbb,
      start_: _fbb.StartTable(),
    }
  }
  // TableInCBuilder &operator=(const TableInCBuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<TableInC> {
    let end = self.fbb_.EndTable(self.start_);
    let o = end as flatbuffers::Offset<TableInC>;
    o
  }
}

#[inline]
fn CreateTableInC(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    refer_to_a1: flatbuffers::Offset<NamespaceA::TableInFirstNS>  /* = 0 */,
    refer_to_a2: flatbuffers::Offset<NamespaceA::SecondTableInA>  /* = 0 */) -> flatbuffers::Offset<TableInC> {
  let mut builder = TableInCBuilder::new(_fbb);
  builder_.add_refer_to_a2(refer_to_a2);
  builder_.add_refer_to_a1(refer_to_a1);
  builder_.Finish()
}

}  // pub mod NamespaceC

pub mod NamespaceA {

impl flatbuffers::Table for SecondTableInA {}
impl SecondTableInA /* private flatbuffers::Table */ {
    const VT_REFER_TO_C: isize = 4;

  fn refer_to_c() -> &NamespaceC::TableInC  {
    self.GetPointer::<&NamespaceC::TableInC>(VT_REFER_TO_C)
  }
  fn mutable_refer_to_c(&mut self) -> &mut NamespaceC::TableInC  {
    /* TODO: are there non-reference choices here? */
    &mut GetPointer::<&mut NamespaceC::TableInC >(VT_REFER_TO_C)
  }
  fn Verify(verifier: &flatbuffers::Verifier) -> bool {
    return VerifyTableStart(verifier) &&
           VerifyOffset(verifier, VT_REFER_TO_C) &&
           verifier.VerifyTable(refer_to_c()) &&
           verifier.EndTable();
  }
}

struct SecondTableInABuilder {
  fbb_: &flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::uoffset_t,
}
impl SecondTableInABuilder {
  fn add_refer_to_c(refer_to_c: flatbuffers::Offset<NamespaceC::TableInC> ) {
    fbb_.AddOffset(SecondTableInA::VT_REFER_TO_C, refer_to_c);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> SecondTableInABuilder {
    SecondTableInABuilder {
      fbb_: _fbb,
      start_: _fbb.StartTable(),
    }
  }
  // SecondTableInABuilder &operator=(const SecondTableInABuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<SecondTableInA> {
    let end = self.fbb_.EndTable(self.start_);
    let o = end as flatbuffers::Offset<SecondTableInA>;
    o
  }
}

#[inline]
fn CreateSecondTableInA(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    refer_to_c: flatbuffers::Offset<NamespaceC::TableInC>  /* = 0 */) -> flatbuffers::Offset<SecondTableInA> {
  let mut builder = SecondTableInABuilder::new(_fbb);
  builder_.add_refer_to_c(refer_to_c);
  builder_.Finish()
}

}  // pub mod NamespaceA

pub mod NamespaceC {

}  // pub mod NamespaceC

pub mod NamespaceA {

#[inline]
fn TableInFirstNSTypeTable() -> &/*mut?*/ flatbuffers::TypeTable {}

}  // pub mod NamespaceA

pub mod NamespaceC {

#[inline]
fn TableInCTypeTable() -> &/*mut?*/ flatbuffers::TypeTable {}

}  // pub mod NamespaceC

pub mod NamespaceA {

#[inline]
fn SecondTableInATypeTable() -> &/*mut?*/ flatbuffers::TypeTable {}

#[inline]
fn TableInFirstNSTypeTable() -> &/*mut?*/flatbuffers::TypeTable {
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 1 },
    { flatbuffers::ET_SEQUENCE, 0, 2 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceA::NamespaceB::TableInNestedNSTypeTable,
    NamespaceA::NamespaceB::EnumInNestedNSTypeTable,
    NamespaceA::NamespaceB::StructInNestedNSTypeTable
  };
  static const char *names[] = {
    "foo_table",
    "foo_enum",
    "foo_struct"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 3, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceA

pub mod NamespaceC {

#[inline]
fn TableInCTypeTable() -> &/*mut?*/flatbuffers::TypeTable {
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 },
    { flatbuffers::ET_SEQUENCE, 0, 1 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceA::TableInFirstNSTypeTable,
    NamespaceA::SecondTableInATypeTable
  };
  static const char *names[] = {
    "refer_to_a1",
    "refer_to_a2"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 2, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceC

pub mod NamespaceA {

#[inline]
fn SecondTableInATypeTable() -> &/*mut?*/flatbuffers::TypeTable {
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceC::TableInCTypeTable
  };
  static const char *names[] = {
    "refer_to_c"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 1, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceA

