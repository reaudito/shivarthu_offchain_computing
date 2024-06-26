type IrohNode = iroh::node::Node<iroh::blobs::store::fs::Store>;
pub struct AppState {
    pub iroh: IrohNode,
}
