use crate::db::DBPool;
use sqlx;

pub async fn sync(conn: DBPool) {
    let rows = sqlx::query!("select id, name from server")
        .fetch_all(&conn)
        .await
        .unwrap();
    // print!("{:?}", rows);

    let user_id: i64 = 109319567435628544;
    
    for row in rows {
        println!("{:?}", row.id);
        sqlx::query!(
            "insert into server_user(server_id, user_id) values(?,?)",
            row.id,
            user_id
        )
        .execute(&conn)
        .await
        .map_err(|e| println!("{}", e)).unwrap();
        
        // match res {
        //     Ok(res) => res,
        //     Err(e) => println!("{:?}", e),
        // }
    }
}
