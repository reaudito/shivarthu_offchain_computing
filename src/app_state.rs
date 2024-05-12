type IrohNode = iroh::node::Node<iroh::bytes::store::fs::Store>;

pub struct AppState {
    pub iroh: IrohNode,
}
