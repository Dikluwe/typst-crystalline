# L0 — stream/text replacement
Hash do Código: 49f95a07

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95

Layer: L3. Exclusive consumer: `03_infra/src/export/stream/text_replacement.rs`.

## Medição anterior à decisão

O lote `00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/`
registra pin vanilla, estado, UTC, fonte, binários e comandos. Trinta quebras
automáticas em oito controles têm identidade U+002D versus U+00AD, mas caixas
coincidentes. `export/fonts.rs:146-154` extrai clusters por GID; identidade
global por fonte/GID não distingue duas ocorrências do mesmo glifo.
Contrato público em `00_nucleo/prompts/entities/layout_types.md` aprovado pelo dono.

## Responsabilidade e aceitação

Unidade privada possui somente abertura e fechamento da substituição PDF por
ocorrência. opening(ops,text) escreve /Span << /ActualText <FEFF…> >> BDC,
codificando todas as unidades UTF-16BE, inclusive pares surrogate e string vazia.
closing(ops) escreve EMC. Nenhum recurso, MCID, transformação, seleção de fonte,
shaping ou desenho nesta unidade. Dispatch top/local desenha os filhos entre
essas chamadas, independentemente de Compact/Verbose e tags.
Não usar escaping WinAnsi ou remapear ToUnicode global. O mesmo GID pode copiar
SHY neste envelope e '-' em outra ocorrência. Testar UTF-16, vazio, escopo
balanceado e percurso Type1 top/local com filhos/hífen literal. Build e lint;
extração fonte→PDF e geometria são aceitação funcional, não bytes completos.

## Substituição e restauração gráfica — refinamento medido

### Medição anterior à decisão

O primeiro build SHA-256 8b223826b2a6ec44bf95a82235e5caaae57777eba27be27200e084e3ce389241
(`after/`, `after-extra/`) conserva SHY no compacto, mas no verboso a extração
coloca os substitutes em x=0/y=400. O experimento controlado de 18 streams em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/envelope-probe/`
refuta a abrangência inicial: fechar EMC antes de Q conserva caixas; fechar após
Q perde a posição no Poppler. Abrir antes/depois de cm/Tm não muda essa conclusão.
Não fechar por contagem de codepoints, nem reescrever streams procurando Q.

### Decisão interna e responsabilidade

Formatter textual conserva suas assinaturas existentes e fornece variantes
privadas com tail de restauração gráfica diferível: emitir todo o texto/ET,
retornar se há Q pendente em vez de restaurar nesse momento. Type1 considera
somente o q de faux-bold; fontes embutidas no verboso têm q próprio; bitmap
não cria tail textual. O dispatcher pede essa forma apenas ao último filho
visual de TextReplacement. Ele fecha as marcações de folha, depois ActualText,
e só então emite Q pendente. Filhos anteriores e itens normais mantêm emissão
existente. Compact/Verbose preservam seus operadores e transformações, sem
troca de modo ou heurística sobre o texto dos operadores.

É refinamento privado de paridade, sem contrato público/fase/flag nova. O owner
text possui formatação/tail, draw possui despacho e marcações de folha, e
text_replacement possui exclusivamente a codificação/BDC/EMC. Aceitação requer
extração textual na posição original em top/local e ambos os modos, coexistência
literal, scopes balanceados e raster preservado.
