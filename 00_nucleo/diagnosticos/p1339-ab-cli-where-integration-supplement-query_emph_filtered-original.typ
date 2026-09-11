*Bold* _Emph_ Text
#context metadata((query(emph).map(it => repr(it.body)), query(emph.where()).map(it => repr(it.body)), query(emph.where(body: [Emph])).map(it => repr(it.body)), query(emph.where(body: [Miss])).len()))
