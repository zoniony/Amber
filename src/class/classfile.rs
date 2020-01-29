
/*
ClassFile {
u4 magic;
u2 minor_version;
u2 major_version;
u2 constant_pool_count;
cp_info constant_pool[constant_pool_count-1];
u2 access_flags;
u2 this_class;
u2 super_class;
u2 interfaces_count;
u2 interfaces[interfaces_count];
u2 fields_count;
field_info fields[fields_count];
u2 methods_count;
method_info methods[methods_count];
u2 attributes_count;
attribute_info attributes[attributes_count];
}
*/
pub struct ClassFile {
    magic                   u32,
    minorVersion            u16,
    majorVersion            u16,
    constantPoolCount       u16,
    constantPool            vec![constantPoolCount],
    accessFlags             u16,
    thisClass               u16,
    superClass              u16,
    
}

impl classfile {
    pub fn new() -> self {
        ClassFile {
            magic:          0,
            minorVersion    0,
            majorVersion    0,
             
        }
    }
}