use mysql::{params, prelude::Queryable};

use crate::{data::authentication::Authentication, time::unixtime_to_datetime};

use super::mysql::get_conn;

pub fn insert_authentication(auth: &Authentication) {
    get_conn()
        .unwrap()
        .exec_batch(
            "INSERT INTO authentications values (:auth_at, :cr_at, :aud, :sub, :met, :prom)",
            std::iter::once(params! {
                "auth_at" => unixtime_to_datetime(auth.authenticated_at),
                "cr_at" => unixtime_to_datetime(auth.created_at),
                "aud" => auth.audience.clone(),
                "sub" => auth.subject.clone(),
                "met" => auth.method.to_string(),
                "prom" => auth.prompt.to_string(),
            }),
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{
        data::authentication::{Authentication, AuthenticationMethod, LoginPrompt},
        time::now,
    };

    #[test]
    #[should_panic]
    fn test() {
        crate::db::_test_init::init_mysql();
        super::insert_authentication(&Authentication::new(
            now(),
            "aud.comame.dev",
            "subject",
            AuthenticationMethod::Password,
            LoginPrompt::Login,
        ));
        todo!()
    }
}
