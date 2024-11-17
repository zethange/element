use axum::{response::Html, routing::get, Extension, Router};
use horrorshow::{helper::doctype, html, Raw};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::user::{Entity, Model};

pub fn get_html_router() -> Router {
    let app = Router::new().route("/", get(get_users));

    app
}

async fn get_users(Extension(db): Extension<DatabaseConnection>) -> Html<String> {
    let users = Entity::find().all(&db).await.unwrap();

    let res = html! {
        : doctype::HTML;
        html {
            head {
                title : "ГООООЛ"
            }
            body {
                h1 {
                    : "Список пользователей"
                }

                form {
                    p {
                        : "Создать пользователя"
                    }
                    input(id = "first_name", placeholder = "Имя") {}
                    input(id = "last_name", placeholder = "Фамилия") {}
                    input(id = "email", placeholder = "Почта") {}
                    input(id = "password", placeholder = "Пароль") {}
                    button(id="submit") {
                        : "Отправить"
                    }
                }

                div {
                    @ for user in users.iter() {
                        div(style = "padding: 10px; border: 1px solid black; border-radius: 10px; margin-bottom: 10px") {
                            p {
                                : format!("Имя: {}", user.first_name)
                            }
                            p {
                                : format!("Фамилия: {}", user.last_name)
                            }
                            p {
                                : format!("Почта: {}", user.email)
                            }
                            p {
                                : format!("Пароль: {}", user.password)
                            }
                        }
                    }
                }
                script {
                    : Raw("
                        const button = document.querySelector('#submit');

                        button.addEventListener('click', async (e) => {
                            e.preventDefault();

                            const [first_name, last_name, email, password] = [
                                document.querySelector('#first_name').value,
                                document.querySelector('#last_name').value,
                                document.querySelector('#email').value,
                                document.querySelector('#password').value,
                            ]

                            const res = await fetch('/api/users', {
                                method: 'POST',
                                body: JSON.stringify({id: 0, first_name, last_name, email, password}),
                                headers: {
                                    'Content-Type': 'application/json',
                                }
                            })

                            if (res.ok) {
                                window.location.reload()
                            } else {
                                const data = await res.text()
                                alert(data)
                            }
                        })
                    ")
                }
            }
        }
    };
    Html(res.to_string())
}
