// #!   format输出文档 https://doc.rust-lang.org/std/fmt/

use std::fmt;

fn main() {
    // 打操作由 std::fmt ⾥⾯定义的一些列
    format!("format！");               // 与printf!类似，不过是返回一个String
    println!("format！");              // 输出到终端：标准输出 io::stdout
    eprintln!("format！");              // 输出到终端：标准错误 io::stderr

    // 按下标，命名参数打印
    println!("{} {}     {b} {a}     {1} {0}",a=100,b=200);
    //        100 200   200 100     200 100

    // 指定类型输出
    println!("{} 输出二进制格式：{0:b} ", 2);

    // 使用0在输出1前面补齐6个0，不写为空格
    println!("{number:>0width$}", number=6, width=6);
    println!("{:06}", 6);


    let pi=3.141592;
    println!("Pi= {:.2}",pi);

    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    println!("DebugPlay: {:?}",DebugPlay(3));

    // 复杂结构需要实现Display trails
    println!("Structure: {}", Structure(format!("format Structure")));
    println!("{:?}", (1,4));

    println!(
        "ComplexDebugPlay: {:#}",
        ComplexDebugPlay{
        age:32,
        name:format!("any"),
        structure:Structure(format!("format Structure"))
        }
    );

    println!("{}",List(vec![DebugPlay(1),DebugPlay(2),DebugPlay(3)]));


    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:#}", *color)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl fmt::Display for Color{
    fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"RGB( {}, {}, {}) 0x{0:0width$X}{1:0width$X}{2:0width$X}",self.red,self.green,self.blue,width=2)
    // println!("{}",Slice(123,format!("哈哈")));

}

#[derive(Debug)]
struct Slice(i32,String);
impl fmt::Display for Slice{
    fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"自定义Display: i32: {}, String: {}",self.0,self.1)
    }
}

struct List(Vec<DebugPlay>);
impl fmt::Display for List{
    fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result{
        let list =&self.0;
        write!(f,"List: 自定义Display: [")?;
        for (index,item) in list.iter().enumerate(){
            if index != 0 { write!(f, ", ")?; }
            write!(f,"{}:{:?}",index,item)?;
        }
        write!(f,"]")
    }
}


#[derive(Debug)]
struct DebugPlay(i32);


#[derive(Debug)]
struct ComplexDebugPlay{
    age:i32,
    name:String,
    structure:Structure
}
impl fmt::Display for ComplexDebugPlay{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "自定义Display: {:#} {:#} {:#}", self.age,self.name,self.structure)
   }
}


#[derive(Debug)]
struct Structure(String);
impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
         write!(f, "{}", self.0)
    }
}
