use mysql::prelude::*;
use mysql::*;

#[test]
fn mysql_test() ->Result<()> {


    #[derive(Debug,Clone)]
    pub struct  Event{
        pub balance: f32,
        pub blocknumber: i32,
        pub    dexaddress:String,
        pub model: String,
        pub serveraddress: String,
        pub transactionhash: String,
        pub useraddress: String,
    }

    //https://www.cnblogs.com/jaciots/p/14761611.html 教程网址

    println!("初始化muysql");
    //设置连接字符串
    let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap();// 类型转换将 url 转为opts
     //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //数据库操作
    println!("初始化muysql");
    //流式查询  数据逐行读取，数据不会存储在内存中
    let result = conn.query_iter("select * from userBuy")
        .unwrap();


    for i in result{
        println!("{:?}",i);
    }
    println!("结果是");

    let event = vec![Event{
        balance: 999.0,
        blocknumber: 102,
        model: "mysql test".to_string(),
        serveraddress: "mysql test".to_string(),
        dexaddress:"dex ".to_string(),
        transactionhash: "mysql 3test".to_string(),
        useraddress: "mysql test".to_string(),
   }]  ;
    
    conn.exec_batch(
        r"INSERT INTO userBuy (balance, blocknumber, model,serveraddress,transactionhash,useraddress,dexaddress)
          VALUES (:balance, :blocknumber, :model, :serveraddress,:transactionhash,:useraddress,:dexaddress)",
        event.iter(). map(|p| params! {
            "balance" => p.balance,
            "blocknumber" => p.blocknumber,
            "model" => &p.model,
            "serveraddress"=>&p.serveraddress,
            "transactionhash"=>&p.transactionhash,
            "useraddress"=>&p.useraddress,
            "dexaddress"=>&p.dexaddress,
        })
    )?;
    

    Ok(())

}



fn mysql_add(){

}

fn mysql_delete(){
    
}

fn mysql_update(){

}

fn mysql_query(){
// Let's select payments from database. Type inference should do the trick here.
// let selected_payments = conn
//     .query_map(
//         "SELECT customer_id, amount, account_name from payment",
//         |(customer_id, amount, account_name)| {
//             Payment { customer_id, amount, account_name }
//         },
//     )?;
}