{ edge, edges }:

edge.capnp {
  src = ./.;
  edges =  with edges.capnp; [ CoreGraphNode ];
  schema = with edges.capnp; ''
    struct CoreGraphListNode {
      list @0 : List(CoreGraphNode);
    }
  '';
}
