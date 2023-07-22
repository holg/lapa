use leptos::{html::Nav, *};
use leptos_router::A;

use crate::{
    auth::{Logout, User},
    routes::AdminRoutes,
};

#[component]
pub fn Layout(cx: Scope, user: User) -> impl IntoView {
    let el_blur: NodeRef<Nav> = create_node_ref(cx);

    let blur = move |_| {
        let _ = el_blur().expect("<nav> to exist").blur();
    };
    view! { cx,
        <div class="admin">
            <header>
                <div class="logo">Admin</div>
                <span>{user.username}</span>
                <Logout/>
            </header>
            <div class="menu" tabindex="0">
                <a class="toggle" href="#" tabindex="0">Menu</a>
                <nav tabindex="1" on:click=blur node_ref=el_blur>
                    <A href="/" exact=true>
                        "Dashboard"
                    </A>
                    <A href="/posts">Posts</A>
                    <A href="/settings">Settings</A>
                </nav>
            </div>
            <main>
                <AdminRoutes/>
            </main>
        </div>
    }
}
