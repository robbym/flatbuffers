// automatically generated by the FlatBuffers compiler, do not modify


extern crate flatbuffers;

mod NamespaceA {
mod NamespaceB {

enum EnumInNestedNS {
  EnumInNestedNS_A = 0,
  EnumInNestedNS_B = 1,
  EnumInNestedNS_C = 2,
  EnumInNestedNS_MIN = EnumInNestedNS_A,
  EnumInNestedNS_MAX = EnumInNestedNS_C
}

const EnumValuesEnumInNestedNS:[EnumInNestedNS; 3] = [
  EnumInNestedNS_A,
  EnumInNestedNS_B,
  EnumInNestedNS_C
];

const EnumNamesEnumInNestedNS:[&'static str; 3] = [
    "A",
    "B",
    "C"
];

fn EnumNameEnumInNestedNS(e: EnumInNestedNS) -> &'static str {
  let index: usize = e as usize;
  EnumNamesEnumInNestedNS[index]
}

// MANUALLY_ALIGNED_STRUCT(4)
#[repr(C, packed)]
pub struct StructInNestedNS {
  i32: a_,
  i32: b_,
} // pub struct StructInNestedNS

impl StructInNestedNS {
  fn Reset(&mut self) {
    memset(this, 0, size_of(StructInNestedNS));
  }
  fn init(&mut self, i32: _a, i32: _b) {
      self.a_ = flatbuffers::EndianScalar(_a);
      self.b_ = flatbuffers::EndianScalar(_b);

  }
  fn a(&self) -> i32  {
    flatbuffers::EndianScalar(self.a_)
  }
  fn mutate_a(&mut self, _a: i32) {
    flatbuffers::WriteScalar(&self.a_, _a);
  }
  fn b(&self) -> i32  {
    flatbuffers::EndianScalar(self.b_)
  }
  fn mutate_b(&mut self, _b: i32) {
    flatbuffers::WriteScalar(&self.b_, _b);
  }
}
// STRUCT_END(StructInNestedNS, 8);

impl flatbuffers::Table for TableInNestedNS {}
impl TableInNestedNS /* private flatbuffers::Table */ {
    const VT_FOO: isize = 4;

  i32 foo() const {
    return GetField<i32>(VT_FOO, 0);
  }
  fn mutate_foo(i32 _foo) -> bool {
    return SetField<i32>(VT_FOO, _foo, 0);
  }
  fn Verify(flatbuffers::Verifier &verifier) -> bool {
    return VerifyTableStart(verifier) &&
           VerifyField<i32>(verifier, VT_FOO) &&
           verifier.EndTable();
  }
};

struct TableInNestedNSBuilder {
  flatbuffers::FlatBufferBuilder &fbb_;
  flatbuffers::uoffset_t start_;
  void add_foo(i32 foo) {
    fbb_.AddElement<i32>(TableInNestedNS::VT_FOO, foo, 0);
  }
  explicit TableInNestedNSBuilder(flatbuffers::FlatBufferBuilder &_fbb)
        : fbb_(_fbb) {
    start_ = fbb_.StartTable();
  }
  TableInNestedNSBuilder &operator=(const TableInNestedNSBuilder &);
  flatbuffers::Offset<TableInNestedNS> Finish() {
    const auto end = fbb_.EndTable(start_);
    auto o = flatbuffers::Offset<TableInNestedNS>(end);
    return o;
  }
};

inline flatbuffers::Offset<TableInNestedNS> CreateTableInNestedNS(
    flatbuffers::FlatBufferBuilder &_fbb,
    i32 foo = 0) {
  TableInNestedNSBuilder builder_(_fbb);
  builder_.add_foo(foo);
  return builder_.Finish();
}

inline flatbuffers::TypeTable *TableInNestedNSTypeTable();

inline flatbuffers::TypeTable *StructInNestedNSTypeTable();

inline flatbuffers::TypeTable *EnumInNestedNSTypeTable() {
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_CHAR, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 0 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    EnumInNestedNSTypeTable
  };
  static const char *names[] = {
    "A",
    "B",
    "C"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_ENUM, 3, type_codes, type_refs, nullptr, names
  };
  return &tt;
}

inline flatbuffers::TypeTable *TableInNestedNSTypeTable() {
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_INT, 0, -1 }
  };
  static const char *names[] = {
    "foo"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 1, type_codes, nullptr, nullptr, names
  };
  return &tt;
}

inline flatbuffers::TypeTable *StructInNestedNSTypeTable() {
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_INT, 0, -1 },
    { flatbuffers::ET_INT, 0, -1 }
  };
  static const int32_t values[] = { 0, 4, 8 };
  static const char *names[] = {
    "a",
    "b"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_STRUCT, 2, type_codes, nullptr, values, names
  };
  return &tt;
}

}  // mod NamespaceB
}  // mod NamespaceA

