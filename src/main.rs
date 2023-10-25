use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "RustConf 2023" }</h1>
            <div>
                <h3>{"Vídeos recomendados"}</h3>
                <p>{ "John Doe: Creando y destruyendo cosas" }</p>
                <p>{ "Jane Smith: El proceso de desarollo" }</p>
                <p>{ "Matt Miller: La Web 7.0" }</p>
                <p>{ "Tom Jerry: Desarrollando solo con teclado" }</p>
            </div>
            <div>
                <h3>{ "John Doe: Creando y destruyendo cosas" }</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="Miniatura del vídeo" />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
