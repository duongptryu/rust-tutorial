#[derive(Debug)]
// struct Member {
//     username: String,
//     email: String,
//     age: u64,
//     active: bool,
// }

struct hinhchunhat {
    dai: u32,
    rong: u32,
}

impl hinhchunhat {
    fn dien_tich(&self) -> u32 {
        self.dai * self.rong
    }

    fn chua(&self, hinh_chu_nhat_khac: &hinhchunhat) -> bool {
        self.dai > hinh_chu_nhat_khac.dai && self.rong > hinh_chu_nhat_khac.rong
    }
}

impl hinhchunhat {
    fn hinhvuong(kichthuoc: u32) -> hinhchunhat {
        hinhchunhat {
            dai: kichthuoc,
            rong: kichthuoc
        }
    } 
}

fn main() {
//     let mut member1 = Member{
//         email: String::from("Test#@gmail.com"),
//         username: String::from("Duong"),
//         age: 28,
//         active: true
//     };

//    let member2 = create_new_member(String::from("PTD"), String::from("duong@gmail.com"), 30);
//    println!("Member 2 = {:#?}", member2);

//    let member3 = Member{
//         username: String::from("join"),
//         ..member2
//    };
//    println!("Member 3 = {:#?}", member3);

   
    // let hinh_chu_nhat = (30,50);
    // println!("Dien tich cua hinh chu nhat = {}", dien_tich(hinh_chu_nhat))

    let kichthuoc = hinhchunhat {
        dai: 30,
        rong: 50,
    };

    let hinh_chu_nhat_2 = hinhchunhat {
        dai: 40,
        rong: 20,
    };
    println!("Dien tich cua hinh chu nhat = {}", dien_tich(&kichthuoc));
    println!("Dien tich cua hinh chu nhat = {}", kichthuoc.dien_tich());
    println!("kich thuoc = {:#?}", kichthuoc);
    println!("Kiem tra = {:#?}", kichthuoc.chua(&hinh_chu_nhat_2))
}

// fn create_new_member(username: String, email: String, age: u64) -> Member {
//     Member {
//         email: email,
//         username: username,
//         age: age,
//         active: true,
//     }
// }

// fn dien_tich (kichthuoc: (u32,u32)) -> u32 {
//     kichthuoc.0 * kichthuoc.1
// }

fn dien_tich (hinhchunhat: &hinhchunhat) -> u32 {
    hinhchunhat.dai * hinhchunhat.rong
}

