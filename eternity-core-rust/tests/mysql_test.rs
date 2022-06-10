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

    mysql_query();
    
    println!("结果是");

    
    

    Ok(())

}



fn mysql_add(){

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

    println!("初始化muysql");
    //设置连接字符串
    let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap();// 类型转换将 url 转为opts
     //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

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
    );

    


}

fn mysql_delete(){
    
}

fn mysql_update(){

}

#[test]
fn test_select_nodeaccountstatus(){
    let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap();// 类型转换将 url 转为opts
     //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();



    //该bug 无法查询tansactionhash的原因是 String 插入会有 “”。
    let mut res:Result<Option<(String,String,String,f32,String,String,String,String)>> = conn
    .exec_first(
        "select * from NodeAccountStatus where balance = :balance",
        params! {
            "balance" => 100.0
        },
    );
    println!("存在数据  {:?}",res.unwrap().unwrap()); 

    

    let mut res:Vec<(String,String,String,f32,String,String,String,String)> = conn
    .query(
        "select * from NodeAccountStatus where optionstatus = 'null' and transactionhash = ".to_string() +&r"'0x8ce9cb9ade8e8fe3dba95c0bc7efa9c157bafbb690d1bbec3f70f4ff8ca5856f'".to_string(), 
    ).unwrap();
    println!("存在数据  {:?}",res); 

    let sql = "select * from NodeAccountStatus where optionstatus = 'null' and transactionhash = ".to_string() +&" '0x8ce9cb9ade8e8fe3dba95c0bc7efa9c157bafbb690d1bbec3f70f4ff8ca5856f' ".to_string();
    println!("{:?}",sql)

    




}

fn mysql_query(){
    println!("初始化muysql");
    println!("初始化muysql");
    let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap();// 类型转换将 url 转为opts
     //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    //数据库操作
    //1.查询user表
    //方式1：流式查询  数据逐行读取，数据不会存储在内存中
    // conn.query_iter("Select * from userBuy  ").unwrap()
    // .for_each(|row|{
    //     let r:(String,i32,String,String,String,String,f32)=from_row(row.unwrap());
    //     println!("{:?}",r);

    //     // is_exist_in_mysql();

    // });




    #[derive(Debug,Clone)]
    pub struct  Event{
        pub balance: f32,
        pub blocknumber: i32,
        pub dexaddress:String,
        pub model: String,
        pub serveraddress: String,
        pub transactionhash: String,
        pub useraddress: String,
    }


    let res = conn
        .exec_first(
            "select * from userBuy where dexaddress = :dexaddress",
            params! {
                "dexaddress" => "dex"
            },
        ).map(
            // Unpack Result
            |row| {
                println!("参数查询的结果是{:?}",&row);
                row.map(|(transactionhash,blocknumber,useraddress,dexaddress,serveraddress,model,balance )| Event {
                    balance:balance,
                    blocknumber :blocknumber,
                    model :model,
                    serveraddress:serveraddress,
                    transactionhash:transactionhash,
                    useraddress:useraddress,
                    dexaddress:dexaddress,
  
                });
            },
        );
   
    println!("{:?}",res);


    


 }