///
/// 空边界泛型 测试用例
///

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red{}
trait Blue{}

impl Red for Cardinal{}
impl Blue for BlueJay{}

fn red<T: Red>(_: &T) -> &'static str {"red"}
fn blue<T: Blue>(_: &T) -> &'static str {"blue"}

#[test]
fn main(){
    let cardinal = Cardinal;
    let bluejay = BlueJay;
    let _turkey = Turkey;

    println!("调用red函数{}", red(&cardinal));
    println!("调用blue函数{}", blue(&bluejay));
}