# Frentes posteriores

- Corrigir o panic 101 ao importar fonte sintaticamente inválida, reproduzido com palavra reservada em invalid-fixture-attempt. A fixture válida das sondas de chamadas não apaga esse defeito; não classificar a tentativa inválida como prova de efeitos.
- Medir/generalizar outras ausências de campos Type e sua grafia de diagnóstico; este lote corrigiu Type::Array medido, sem normalizar silenciosamente os erros restantes.
- As sondas negativas matemáticas divergentes são preservadas em observables-result.json; não equivalem a cobertura de construção de frac/vec ou coerção de ints.
- O inventário global de V16/V19/V20 e as frentes RG1, FG1, RM1, SY1, igualdade std entre arquivos e demais itens anteriores continuam abertos. RG1 exige aprovação própria; candidatos futuros devem ser rebaseados sobre o estado atual.
- A classificação interna de Func/Type versus outras variantes testa o contrato já existente de chamada; ela não prova toda capacidade de cast upstream, nem fecha hints de shadowing ou identidade/morfologia de Symbol.
- Não houve commit, push, merge, release ou supressão de lint. O objetivo global permanece ativo.
