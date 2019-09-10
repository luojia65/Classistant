pub(crate) fn connect(params: Vec<&str>) -> mysql::Pool {
    let mut builder = mysql::OptsBuilder::new();
    builder.ip_or_hostname(Some(params[0]));
    builder.tcp_port(params[1].parse().expect("parse port"));
    builder.user(Some(params[2]));
    builder.pass(Some(params[3]));
    builder.db_name(Some(params[4]));
    mysql::Pool::new(builder)
        .expect("connect to mysql server")
}
