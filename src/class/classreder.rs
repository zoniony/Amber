/*
func Parse(classData []byte) (cf *ClassFile, err error) {...}
func (self *ClassFile) read(reader *ClassReader) {...}
func (self *ClassFile) readAndCheckMagic(reader *ClassReader) {...}
func (self *ClassFile) readAndCheckVersion(reader *ClassReader) {...}
func (self *ClassFile) MinorVersion() uint16 {...} // getter
func (self *ClassFile) MajorVersion() uint16 {...} // getter
func (self *ClassFile) ConstantPool() ConstantPool {...} // getter
func (self *ClassFile) AccessFlags() uint16 {...} // getter
func (self *ClassFile) Fields() []*MemberInfo {...} // getter
func (self *ClassFile) Methods() []*MemberInfo {...} // getter
func (self *ClassFile) ClassName() string {...}
func (self *ClassFile) SuperClassName() string {...}
func (self *ClassFile) InterfaceNames() []string {...}
*/

use classfile::ClassFile;
use std::fs::File;
use std::io::BufReader;

struct  ClassReader {
    bufreader: BufReader<File>,
}

impl ClassReader -> Result<bufreader,Error> {
    pub fn new(filename: &str) {
        let file = File::open(filename)?;
        Ok(
            ClassFileReader {
                bufreader: BufReader::new(file)
            }
        )
    }

    pub read() -> Result<> {
        
    }

    fn read_magic() -> Result<> {
        magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!")
        }
    }

    fn read_version() -> Result<> {

    }


}
