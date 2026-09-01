# Recibo independente — C-P1284-v7

**Estado:** `SEALED_SPEC_FOR_IMPLEMENTATION`

Este recibo sela a especificação observável do Passo 1284 e separa o subconjunto
autorizado para fluxo contínuo das superfícies bloqueadas pelo ADR-0127. Não é
certificado de implementação, não afirma equivalência funcional geral e não
afirma que a matriz de mutações já foi executada.

`C-P1284-v1` foi invalidado antes da implementação: a auditoria do contrato
público cristalino mostrou que `Selector` possui `And`, `Or` e `Within`, mas não
possui `Before` nem `After`. `C-P1284-v2` foi então usado como contrato causal
para atualizar os L0 proprietários antes de código; essa alteração protegida
invalidou seu hash. `C-P1284-v3` foi invalidado antes de código candidato pelo
oráculo independente A-P1284-v2: `rocket`/`mako` têm 256 cores, não 246/252.
`C-P1284-v4` foi invalidado quando a execução A-P1284-v3 revelou duas causas
L0 ausentes: chamabilidade de `Type::Arguments` e cast Ratio de `cmyk`. Este v5
substituiu v1–v4. A execução posterior A-P1284-v4 avançou além de `cmyk` e
revelou que v5 ainda não fixava os casts heterogéneos de HSL, HSV, Oklab e
Oklch. Este v6 substitui integralmente v1–v5, preserva a matriz de paths e
acrescenta obrigações e mutações discriminatórias para essas quatro formas.
Após a implementação, `crystalline-lint --fix-hashes .` alterou sete entradas
L0 protegidas. A auditoria mecânica de §2.1 confirmou que apenas a linha
`Hash do Código` mudou em cada uma; este v7 substitui v6 somente para repinar
essas identidades, sem qualquer mudança normativa ou observável.

## 1. Manifesto e identidade

- Papel: autor do contrato, sem autoridade de implementação, autoria de testes,
  ataque ou veredito final.
- Regime: protocolo completo de materialização segregada.
- Instante do selo v7: `2026-08-30T12:39:06-03:00`.
- HEAD informativo: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Estado pré-candidato: identificado pelos hashes integrais já congelados. O
  consumer `foundations/color.rs`, descoberto depois de existir candidato, não
  foi aberto; sua identidade disponível vem do L0, conforme §2.
- Vanilla ratificado: upstream/main `a51e02804`.
- Binário candidato congelado pelo verificador, não aberto por esta autoria:
  SHA-256 `8b85f933b7cd1fa74e46e2c18902b8343d9064f11b2a76844a476a252835a57e`.

Entradas de coordenação:

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `00_nucleo/materialization/typst-passo-1284.md` | `3c42356b485289920af3ad2e9161c4face98ee928c38da06fe41c4aa608eee64` |
| `00_nucleo/diagnosticos/p1283-residual-p1284.json` | `42d99bb0bed744d8266cfee185b759eba781230e7ff8ac5a2dde64153fde7c97` |
| Contrato predecessor `C-P1284-v2` | `5a9bff30be181d0f6d16aa71055422a6faaca9a99bc13039d51528b5d2657ca7` |
| Contrato predecessor `C-P1284-v3` | `ddfe762151c6b9cda025f3d4698f8982ec19160ad363566f86a9bd6748e00cb3` |
| Contrato predecessor `C-P1284-v4` | `359ef5ae601b1bc1496bb7ff478d4b1fd8a41649321464c2f43a12a2db8dbc88` |
| Contrato predecessor `C-P1284-v5` | `f9a9987c2d4220943d39da97074a6f00c6c0e4e162e0c171ca1535561cbfd85c` |
| Contrato predecessor `C-P1284-v6` | `dca90d681ed1b4b9cc6661cfbff05aa95f37d821bf9f9bb613f90681c318543c` |
| `lab/surface-inventory/p1284-probes.json` (A-P1284-v2, 52/52) | `06e205483a9bc905f778a72fcbb74ddb229b73141ffe6bf27ec95d0a40e145c3` |
| `00_nucleo/diagnosticos/p1284-oracle-receipt.md` | `50b6e34b826041edb61f15828a14c2cfd234bc7bee71215c378e89e68079ba79` |
| `lab/surface-inventory/p1284-probes.json` (A-P1284-v3) | `75808a72e5525dfd7780c66fa27b0238a90dbc2dc8cfab491defd942e70d39eb` |
| `00_nucleo/diagnosticos/p1284-oracle-receipt.md` (A-P1284-v3) | `b350e8b30127b1cf2d280c1bd4b9ad7a7c33153ff067a666a369797293fe2671` |
| `lab/surface-inventory/p1284-probes.json` (A-P1284-v4) | `9f9cb81265e1eaf3215568b29dd85705e728b014bed7e9e52f62f62af91b52a5` |
| `00_nucleo/diagnosticos/p1284-oracle-receipt.md` (A-P1284-v4, invalidado aguardando v6) | `42adb7832331bb6200e5b02150545585056d0ab92572ea775265f5fd96266447` |

Prompts L0 lidos e congelados:

| Owner L0 | SHA-256 |
|---|---|
| `compiler/stdlib/collections.md` | `f474e4b654509de4c5dda75223499463388d2e683be36b4eff11dbaf732839e5` |
| `compiler/eval/bindings/method_dispatch.md` | `fb6ca8f6482d08ad0dde8d900bdb882a66f7abafebb2761d3a4b226d6589e951` |
| `compiler/eval/bindings/field_access.md` | `97122927d00c44fac5039b1ae45fc1705e6f6a4743fa5d31e8a7eedbe7e2e649` |
| `compiler/eval/call_dispatch.md` | `9dfe195c585fa95acf71d0c2dec41541bf16b72dac4033c55a83159f8b68809f` |
| `compiler/eval/bindings/value_methods.md` | `85d31916ee6091f9853e2605760c3ecc41d4be5c5a702f1fa7e4b3bacc1b6091` |
| `compiler/stdlib/foundations/str.md` | `4b3fa3d5cb2c313fa13326dc8c398f70ef89efb229a6a8ae8b37c1e77bca0043` |
| `compiler/stdlib/foundations/color.md` | `453059c00c2acefcb7c8610523e986a98edd1ed7a0a1b36fb24dcd357fa5b048` |
| `compiler/stdlib/color.md` | `7149cf1a93c96dab1c4d31aa491fd2093bfccd06459681c6f9f04ee1eff2677c` |
| `entities/color.md` | `41d6aaa1e696de39823ca63fe179351125a8573467521c1a0e6d795afdb4562a` |
| `entities/dir.md` | `b9cd0f95837004169ec06f16ab6c9fb7b138f1407969e3da42e3587d72ae4aba` |
| `entities/layout_types.md` | `68f453b4abfd333f17f6f989ac83e11846926b9b91b16e70ab455bb17f603779` |
| `entities/args.md` | `3a33f1f2628348e3484b8346ab55aa82474b789e6836c62303b24b43297f5d56` |
| `entities/duration.md` | `b16ff16253af6ad0cd119679880a429f401bd3815cebb7077f70179bc1c6ae2e` |
| `compiler/stdlib/primitives-constructors/duration.md` | `c7112b9eddb05d6cd885da3a4e269709ae2467f74a7ca7f6144f3b3531f88454` |
| `entities/selector.md` | `8dcd7bfbc6789ed0ef669d8ffd943df63aa06343f0c4ef39f23e797ec276b429` |
| `compiler/stdlib/foundations/selector.md` | `4e9d874ee11ba8f89aca7999881558d1981c677a6eceeee4afa03da3ea62e46d` |
| `entities/state.md` | `466e16e018de4bd1cc043e1cf79b92bf5c4b375b178a65ec85228e373bdecec4` |
| `compiler/stdlib/state.md` | `daf180d5e77f0a9d307b8b1a2b148741a6ac82a776ab17a510f107c5bff42eb1` |
| `entities/location.md` | `4628ee19327ebe7eb60b13fe010c526e378b11efc76728b878de643d0a188883` |
| `entities/elements/outline.md` | `58ea2a650fdedd81fa267c902f5387ea1a4cfe42e381b9afa13497c486b19fc4` |
| `compiler/stdlib/structural/outline.md` | `e8dc3b96fd9a03ae21f87f602d85bb7b0b5b6eb95d6267dcbfe280a80d3b0632` |
| `compiler/stdlib/pdf.md` | `5d32e0013e91b42943b99276f2f334000ed85c60d71147b950c1b1bd645b9f4b` |
| `entities/bytes.md` | `bd6199460444e5c82754a5d7050733bd0a5c36bf58d907855287fd1b6b539a7f` |

Consumers cristalinos pré-candidato:

| Consumer | SHA-256 |
|---|---|
| `compiler/stdlib/collections.rs` | `c4d31a30205044c09dcccdee7aac3de9688515a561f1a4eea8799fb95aad4fe8` |
| `compiler/eval/bindings/method_dispatch.rs` | `e41bfcd63c8add205a620ed999e42d61b26c1a2f72d439c62f1deae786f849bb` |
| `compiler/eval/bindings/field_access.rs` | `444fc462a38260a07dfbfb460e5b02efc5c016c2290c72618ce242f184e92e67` |
| `compiler/eval/call_dispatch.rs` | `20c89e29f6e82868550bc3097c775ebaa2d58f4df04bc7ec04e8abded4414a9e` |
| `compiler/eval/bindings/value_methods.rs` | `bc4160130efbe4b4d98dcd02f0368bf8ce0c13a306ce16450dab224c54ed8a17` |
| `compiler/stdlib/foundations/str.rs` | `8d24b7d7df2f0c44538e6ad05f6d4da5276a617758536513e793db874956e8db` |
| `compiler/stdlib/color.rs` | `60874dbcb7a1bda9645d518240dc5a53696aa9065f67b72b22cd52cc125c8566` |
| `entities/color.rs` | `e0bfe7c90da2c101afae00d6eea7311730aef0a789f43ccac110772a5eb0b3ba` |
| `entities/dir.rs` | `d17f3e011c62f78456ecca5367bc4f5e626753fae52915abcb3c37a55126b94e` |
| `entities/layout_types.rs` | `da93d06308d3cfc9a6667e9391a83eebfaaa77a2761f4f1f489356f4f9159a17` |
| `entities/args.rs` | `c4390cee53010c891656e33dc08159a597f99b10f8f9a1a54e9eb67acd15155a` |
| `entities/duration.rs` | `2ba6f8c14f7429b174e1754e358515c43745e5242d665a5d3e2bb143906fd560` |
| `compiler/stdlib/primitives_constructors/duration.rs` | `36a7489e6f4fe2eec795867df2c26b46cbe9b77495ddc0b5e1cd2ebffdbde538` |
| `entities/selector.rs` | `77780eee221ee71ac5555dfc3cca51113d7d9e2234f7540474f4e421565baf4a` |
| `compiler/stdlib/foundations/selector.rs` | `8833d8475a714c1e14d9c85cea9512fed9519947e7c2201c42f405714adb2afe` |
| `entities/state.rs` | `db06762e151c1501904828d59f8df4a282a5df5bf8082855d5576d8a8aa83e23` |
| `compiler/stdlib/state.rs` | `d7e8190f95d6511679c0f0b870bbbce7efc85506a43b5e86d871935a10e117ef` |
| `entities/location.rs` | `fca2f39716b5cb674ed6417884fe0a28a2c02dc7123d4888af4dfb4519dd4f06` |
| `entities/elements/outline.rs` | `5fd553d299d83c43ec6ea901aec444155b199e78d14988035356b22cafb43491` |
| `compiler/stdlib/structural/outline.rs` | `b2e0ee47ccaaf0dad2bacbdb8ee7aa90c7c18bea02cb63f5a522157fe9d6df87` |
| `compiler/stdlib/pdf.rs` | `095d2b030d17134ad860c5acc303c06b5bfe76243b1190ba8b033b09e3315346` |
| `entities/bytes.rs` | `8d71b928732a0771e980651d72bfc4c6c3aa285062cc9eb99bcab1cb288cecf7` |

As fontes vanilla ratificadas relevantes foram fixadas por hashes integrais:
`array.rs f1a92015…f5d35d`, `bytes.rs 9307030e…86929`,
`args.rs b681b149…849b`, `str.rs 8648ad25…cdb7`,
`dict.rs 43b5a6bc…f9f4b`, `color.rs 80473eba…0713`,
`dir.rs 2887408c…50f4`, `align.rs e0ad22c9…2993`,
`duration.rs 42504ed7…ee0`, `length.rs 7c74ad8d…61de`,
`selector.rs 84615405…fbdc`, `state.rs ea7e29c2…767f`,
`location.rs 76ddfc85…390`, `outline.rs 74c22ad7…fd48`,
`pdf/accessibility.rs dba1d5f5…7d51` e `pdf/attach.rs 525d2505…aed`.

## 2. Ownership 1:1

### 2.1 Refutação de mudança normativa no resselo pós-implementação

Os 23 L0 finais foram recalculados a partir do workspace. Dezasseis coincidem
integralmente com os pins C-v6. Nos sete restantes, a auditoria substituiu em
memória exclusivamente a linha `Hash do Código` atual pelo valor anterior e
recalculou SHA-256, sem editar ficheiro nem abrir código candidato:

| L0 | Pin integral C-v6 | Header C-v6 → atual | SHA reconstruído |
|---|---|---|---|
| `compiler/stdlib/collections.md` | `0df6b0d1610cdd5bff2d5052e6fcc992b488894a919af8a89ae462c44f157dc3` | `42727884` → `fdc334c2` | igual ao pin C-v6 |
| `compiler/eval/bindings/field_access.md` | `4a25fb34947acfe5cb26392cba69f1ceea31e53073cbc68f8350f359f40d7b98` | `6f7ae61e` → `4e71df02` | igual ao pin C-v6 |
| `compiler/eval/call_dispatch.md` | `57cbd2be4a5f6602d5f4c50f405f4c9326cd4bdc6d35bf166ede57b81c686e28` | `2fbc1511` → `cdeb74dd` | igual ao pin C-v6 |
| `compiler/eval/bindings/value_methods.md` | `d2c19454df0b4e4bc846cb88dd1c12ae92a40518056f06d0fa5105a3c4cba67c` | `cf8dcf06` → `def19f1d` | igual ao pin C-v6 |
| `compiler/stdlib/foundations/str.md` | `3c467bbf1d6ee0757cb48bca67bbdef8aaf2c6f1ddeb917bc2d5d31a5c364957` | `f824b2f2` → `890a94d9` | igual ao pin C-v6 |
| `compiler/stdlib/foundations/color.md` | `e811f2fdb273192a4e60cd75ff35042fe0a9bba5989469c5be8969022370729e` | `3caa644c` → `f488e748` | igual ao pin C-v6 |
| `compiler/stdlib/color.md` | `259445b2b95e2d634da3eceb8ad1c3a8bb5a98d2db0238375d40f3d089a136c1` | `88be1393` → `9a2e6451` | igual ao pin C-v6 |

Forma reproduzível usada em cada linha:
`sed 's/^Hash do Código:.*/Hash do Código: <valor-C-v6>/' <L0-atual> | sha256sum`.
O resultado foi comparado com o pin integral C-v6 da segunda coluna.

Essa igualdade refuta mudança normativa: a transformação de auditoria toca
uma única linha conhecida; qualquer byte diferente no corpo, ownership, pin de
Núcleo, obrigação, exceção ou critério impediria a coincidência com o SHA
C-v6. Os SHA atuais dos sete estão pinados em §1 e os outros dezasseis
permanecem literalmente iguais. Logo observáveis, `Unknown`, bloqueios
ADR-0127, matriz 35+10 e ownership de C-v6 são semanticamente idênticos neste
v7. O binário candidato permanece congelado no SHA declarado em §1 e não foi
lido nem recalculado por esta autoria.

### 2.2 Ownership preservado

Cada linha da tabela L0/consumer acima é uma relação de propriedade 1:1.
Referências adicionais em headers antigos, como `eval/mod.rs` para registo de
`pdf` ou `value.rs` para alojar uma variante, são callsites/co-mudanças, não
consumers proprietários. Antes do resselo, os L0 afetados devem tornar essa
distinção explícita; V15 e V26 precisam estar verdes.

O núcleo `state/language-semantics.toml` permanece partilhado apenas pelos
owners `entities/state` e `stdlib/state`, com o pin já declarado em ambos. Este
contrato não cria ownership direto de código por Núcleo Tekt.

P1284 não criou nova unidade de glue: `field_access` possui somente lookup,
`call_dispatch`/`value_methods` somente orchestration e cada operação delega ao
owner semântico. Ao longo dos ressellos foram atualizados dez L0 existentes;
nenhuma relação 1:N foi introduzida.

As duas pré-condições acrescentadas por v5 têm ownership inequívoco:

- `compiler/eval/call_dispatch.md` ↔
  `01_core/src/compiler/eval/call_dispatch.rs`: owner exclusivo da rota que
  torna `Type::Arguments` chamável. Os métodos de `arguments` continuam
  pertencendo a `compiler/stdlib/collections.md`; não há semântica duplicada.
  O consumer pré-candidato permanece identificado na tabela por
  `20c89e29…a1a9e` e o L0 v7 por
  `9dfe195c585fa95acf71d0c2dec41541bf16b72dac4033c55a83159f8b68809f`.
- `compiler/stdlib/foundations/color.md` ↔
  `01_core/src/compiler/stdlib/foundations/color.rs`: owner exclusivo dos
  casts/construtores nativos, inclusive os quatro componentes Ratio de
  `cmyk`; `compiler/stdlib/color.md` só publica/delega o namespace e alias.
  Por isolamento procedimental, esse consumer não foi aberto após existir
  candidato. A identidade congelada disponível é o `Hash do Código` curto
  `f488e748` declarado pelo L0 e o hash integral do próprio L0 v7
  `453059c00c2acefcb7c8610523e986a98edd1ed7a0a1b36fb24dcd357fa5b048`;
  este contrato não faz alegação sobre bytes candidatos.

## 3. Relação observável

Para cada path selecionado:

```text
(path, owner, existência, classe de acesso, kind, valor/repr,
 assinatura e metadata, chamada real, resultado/erro/contexto,
 identidade de alias ou módulo, classificação ADR-0127)
```

`instance_method` significa duas formas coerentes: `value.method(...)` e a
projeção não ligada `type.method(value, ...)`. A projeção recebe `self` como
primeiro positional e delega à mesma unidade semântica. `type_static_member`
sem `self` é função estática ou constante, conforme o kind. Presença do nome
sem chamada real nunca satisfaz o contrato.

## 4. Inventário completo congelado

Os perfis `default` e `html` possuem o mesmo conjunto de 180 paths P1284:
115 `MISSING_MEMBER`, 44 `UNVERIFIED_METADATA` e 21 `UNKNOWN`; não há
`MISSING_BINDING` nem `WRONG_KIND` nesse recorte. As contagens vêm do residual
pinado acima, medido em working tree não commitada em
`2026-08-30T13:25:23.068645+00:00`.

Notação: `i` = método de instância + projeção não ligada; `s` = função estática;
`c` = constante; `m` = módulo/membro; `b` = binding. `M`, `U` e `X` são as
classificações residuais `MISSING_MEMBER`, `UNVERIFIED_METADATA` e `UNKNOWN`.

- **array — 33:** `all(self,test)` `[i/U]`; `any(self,test)`,
  `at(self,index,default:?)`, `chunks(self,chunk-size,exact:false)`,
  `contains(self,value)`, `dedup(self,key:?)`, `enumerate(self,start:0)`,
  `filter(self,test)`, `find(self,searcher)`, `first(self,default:?)`,
  `flatten(self)`, `fold(self,init,folder)`, `insert(self,index,value)`,
  `intersperse(self,separator)`, `join(self,separator:none,last:?,default:none)`,
  `last(self,default:?)`, `len(self)`, `map(self,mapper)`, `pop(self)`,
  `position(self,searcher)`, `product(self,default:?)`, `push(self,value)`,
  `reduce(self,reducer)`, `remove(self,index,default:?)`, `rev(self)`,
  `slice(self,start,end:none,count:?)`, `sorted(self,key:?,by:?)`,
  `split(self,at)`, `sum(self,default:?)`, `to-dict(self)`,
  `windows(self,window-size)`, `zip(self,exact:false,..others)` `[i/M]`;
  `range(start:0,end,inclusive:false,step:1)` `[s/M]`.
- **bytes — 3:** `at(self,index,default:?)`, `len(self)`,
  `slice(self,start,end:none,count:?)` `[i/M]`.
- **arguments — 6:** `at(self,key,default:?)`, `filter(self,test)`,
  `len(self)`, `map(self,mapper)`, `named(self)`, `pos(self)` `[i/M]`.
- **str — 21:** `clusters(self)` `[i/U]`; `from-unicode(value)` `[s/U]`;
  `at(self,index,default:?)`, `codepoints(self)`, `contains(self,pattern)`,
  `ends-with(self,pattern)`, `find(self,pattern)`, `first(self,default:?)`,
  `last(self,default:?)`, `len(self)`, `match(self,pattern)`,
  `matches(self,pattern)`, `normalize(self,form:"nfc")`,
  `position(self,pattern)`, `replace(self,pattern,replacement,count:?)`,
  `rev(self)`, `slice(self,start,end:none,count:?)`,
  `split(self,pattern:none)`, `starts-with(self,pattern)`,
  `trim(self,pattern:none,at:?,repeat:true)` `[i/M]`;
  `to-unicode(character)` `[s/M]`.
- **dictionary — 9:** `at(self,key,default:?)`, `filter(self,test)`,
  `insert(self,key,value)`, `keys(self)`, `len(self)`, `map(self,mapper)`,
  `pairs(self)`, `remove(self,key,default:?)`, `values(self)` `[i/M]`.
- **color — 56:** constantes `aqua`, `black`, `blue`, `eastern`, `fuchsia`,
  `gray`, `green`, `lime`, `maroon`, `navy`, `olive`, `orange`, `purple`,
  `red`, `silver`, `teal`, `white`, `yellow` `[c/U]`; estáticas `cmyk`,
  `hsl`, `hsv`, `linear-rgb`, `luma`, `mix`, `oklab`, `oklch`, `rgb` `[s/U]`;
  instância `components`, `darken`, `desaturate`, `lighten`, `negate`,
  `opacify`, `rotate`, `saturate`, `space`, `to-hex`, `transparentize`
  `[i/U]`; `map` `[m/M]`; `spot` `[tipo-estático/M]`; mapas `cividis`,
  `coolwarm`, `crest`, `flare`, `icefire`, `inferno`, `magma`, `mako`,
  `plasma`, `rainbow`, `rocket`, `spectral`, `turbo`, `viridis`, `vlag`
  `[m/X, kind=array]`; `spot.tint(self,value)` `[i/X]`.
- **direction — 11:** `axis(self)`, `end(self)`, `inv(self)`, `sign(self)`,
  `start(self)` `[i/M]`; `from(side)`, `to(side)` `[s/M]`; `btt`, `ltr`,
  `rtl`, `ttb` `[c/M]`.
- **alignment — 10:** `axis(self)`, `inv(self)` `[i/M]`; `bottom`, `center`,
  `end`, `horizon`, `left`, `right`, `start`, `top` `[c/M]`.
- **duration — 5:** `days(self)`, `hours(self)`, `minutes(self)`,
  `seconds(self)`, `weeks(self)` `[i/M]`.
- **length — 5:** `cm(self)`, `inches(self)`, `mm(self)`, `pt(self)`,
  `to-absolute(self)` `[i/M]`.
- **selector — 5:** `after(self,start,inclusive:true)` e
  `before(self,end,inclusive:true)`
  `[i/M, BLOCKED_ADR0127_PUBLIC_CONTRACT]`; `and(self,..others)`,
  `or(self,..others)` e `within(self,ancestor)` `[i/M, fluxo contínuo]`.
- **state — 4:** `at(self,selector)`, `final(self)`, `get(self)`,
  `update(self,update)` `[i/M]`.
- **location — 3:** `page(self)`, `page-numbering(self)`, `position(self)`
  `[i/M]`.
- **outline — 7:** binding
  `outline(title:auto,target:heading,depth:none,indent:auto)` `[b/U]`;
  `entry(level,element,fill:repeat(body:[.],gap:0.15em))` `[m/M]`;
  `entry.body(self)`, `entry.inner(self)`, `entry.page(self)`,
  `entry.prefix(self)`, `entry.indented(self,prefix,inner,gap:0.5em)` `[m/X]`.
- **pdf — 2:** `artifact(kind:"other",body)` e
  `attach(path,data,relationship:none,mime-type:none,description:none)` `[m/U]`.

## 5. Kind, módulos, constantes e aliases

- `color.map` é `module`; cada um dos seus 15 membros é `array` e preserva
  integralmente quantidade, ordem e valor de cada cor do preset vanilla.
  Em particular, `rocket` e `mako` têm ambos 256 itens; primeiros
  `0x03051aff`/`0x0b0405ff`; digests canônicos
  `b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b`
  e `f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77`.
- `color.spot` é `type`, não módulo nem função comum. Seu resultado é
  `SpotColorant`; `.tint` devolve uma cor spot.
- `outline.entry` é uma função-elemento com scope próprio, não um dicionário
  nem módulo genérico. Seus cinco subfields são funções no scope desse elemento.
- `pdf` permanece módulo com exatamente `artifact` e `attach` no perfil padrão.
- `selector.and`, `selector.or` e `selector.within` projetam as variantes
  públicas já existentes `Selector::And`, `Selector::Or` e `Selector::Within`.
  `selector.before`/`after` não têm variante pública correspondente e não podem
  ser representados por aliases, composição aproximada ou abuso dessas três.
- Direções e alinhamentos são constantes do tipo correspondente. As formas
  qualificadas são aliases de linguagem dos globals já existentes:
  `direction.rtl == rtl`, `alignment.left == left`, etc.
- As 18 cores qualificadas são aliases dos bindings globais ratificados e
  devem reutilizar a mesma tabela, inclusive precisão `f32` e espaço Luma/RGB.
- Constructors/operadores qualificados de `color` e globais delegam à mesma
  função e preservam `repr`; igualdade de valor casual nunca cria alias novo.
- Métodos não ligados não são constantes: resolvem para `function`, recebem
  `self` primeiro e preservam a semântica da forma ligada.

## 6. Sentinelas e chamadas reais

Os seis sentinelas do passo são obrigatórios, mas não substituem as 180 linhas:

```text
array.len((1, 2, 3))                  -> 3; kind function
bytes.len(bytes("é"))                 -> 2; kind function
arguments.len(arguments(1, x: 2))    -> 2; conta positional e named
type(arguments)                       -> type; `arguments` é chamável
arguments(1, x: 2).pos()/named()      -> (1,) / (x: 2)
alignment.left                       -> left; kind alignment; == left
direction.rtl                        -> rtl; kind direction; == rtl
color.map                            -> module
color.map.viridis                    -> array; primeiro #440154, último #fee825
color.cmyk(0%, 0%, 0%, 100%)         -> color CMYK; quatro Ratio posicionais
color.hsl(0deg, 100%, 50%)            -> color HSL; Angle + Component + Component
color.hsv(0deg, 100%, 100%)           -> color HSV; Angle + Component + Component
color.oklab(50%, 0, 0)                -> color Oklab; Ratio + dois Chroma
color.oklch(50%, 0, 0deg)             -> color Oklch; Ratio + Chroma + Angle
str.from-unicode(0)                  -> string de um scalar U+0000
str.to-unicode(str.from-unicode(0))  -> 0
```

`str.from-unicode` rejeita somente valor fora de Unicode scalar/surrogate;
U+0000 é válido no baseline ratificado. Esta precisão refuta a redação L0
histórica que proibia `\0`, sem alterar o inventário de paths.

### Refutação da medição v3 de `rocket`/`mako`

A contagem 246/252 não vinha de truncamento do arquivo vanilla. O extrator do
autor v3 aceitava somente `0x` seguido de exatamente oito dígitos. Na fonte,
`rocket` contém 10 literais de sete dígitos e 246 de oito; `mako`, 4 de sete e
252 de oito. O zero inicial omitido é válido em literal Rust e reaparece no
valor público (`0x03051aff`, `0x0b0405ff`). Assim, o extrator descartou
exatamente 10/4 entradas. A regra corrigida aceita 1–8 dígitos, interpreta o
inteiro e formata lowercase `0x%08x` antes de contar e calcular SHA-256.

A-P1284-v2 confirmou 15/15 mapas e 52/52 sondas no vanilla. Esta evidência
refuta a hipótese v3 e torna cardinalidade, primeiro, último e digest dos dois
mapas observáveis obrigatórios. Mutar cardinalidade, prefixo omitido ou digest
cai nas mutações 18–19.

### Auditoria v5 — pré-condições `arguments` e `cmyk`

As duas lacunas foram confirmadas antes de qualquer leitura de candidato. Em
`lab/typst-original/crates/typst-library/src/foundations/args.rs:320-340`, o
vanilla ratificado declara `arguments` como `#[func(constructor)]`, variádico,
e devolve o `Args` de entrada preservando posicionais e named. A mera existência
de `Type::Arguments`/`Value::Args` não satisfaz esse observável se o dispatch de
chamada continuar ausente. O L0 `call_dispatch.md:272-309` agora obriga a rota
chamável sem mudar o contrato Rust público de `Args`.

Em `lab/typst-original/crates/typst-library/src/visualize/color.rs:649-680`, os
quatro parâmetros da forma sentinela de `cmyk` são `RatioComponent`; em
`:2650-2663`, esse cast aceita Ratio entre 0% e 100%, não Float/Int. Logo
`color.cmyk(0%, 0%, 0%, 100%)` e a forma global homóloga devem resolver pela
mesma nativa do owner `foundations/color`, preservando ordem CMYK e validação.
O L0 `foundations/color.md:43-68` foi corrigido antes deste resselo.

Ambas são correções de paridade em representação já existente: seguem fluxo
contínuo pelo ADR-0127 e não autorizam variante/campo/método público, mudança de
default ou fase de pipeline.

### Auditoria v6 — casts heterogéneos dos quatro constructors de cor

A sonda A-P1284-v4 pinada acima já continha e obteve sucesso vanilla para
`hsl(0deg,100%,50%)`, `hsv(0deg,100%,100%)`, `oklab(50%,0,0)` e
`oklch(50%,0,0deg)`. A fonte ratificada confirma que não existe um cast comum
Float/Int para essas formas:

- `hsl`/`hsv` recebem `Angle`, dois `Component` e alpha `Component` opcional
  com default 100%; `Component` aceita Int 0–255 ou Ratio 0%–100%, nunca
  Float;
- `oklab` recebe `RatioComponent`, dois `ChromaComponent` e alpha
  `RatioComponent` opcional com default 100%;
- `oklch` recebe `RatioComponent`, `ChromaComponent`, `Angle` e alpha
  `RatioComponent` opcional com default 100%;
- `RatioComponent` aceita apenas Ratio 0%–100%; `ChromaComponent` aceita Float
  (incluindo Int coerçível para `f64`) literalmente ou Ratio escalado por 0,4,
  sem clamp adicional; `Angle` aceita `deg`/`rad` sem range angular de rejeição
  específico do constructor.

As medições normativas são `visualize/color.rs:393-489`, `:702-793` e
`:2651-2691`, mais `foundations/value.rs:619` e `layout/angle.rs:12-17`.
O L0 owner `foundations/color.md:71-136` agora fixa tipos, unidades, ranges,
ordem, defaults, aliases e erros antes deste resselo. A correção continua no
ADR-0127, sem novo contrato Rust público, default de produto ou fase.

Para todo método `i`, a suíte posterior deve comparar chamada ligada e não
ligada com entrada válida, argumento em falta, positional excedente, named
desconhecido, tipo errado e pelo menos um caso de default. Funções variádicas
testam zero/um/múltiplos; métodos contextuais (`state`, `location`,
`selector.within`, `length.to-absolute`) testam contexto presente e ausente.
Constantes testam kind, valor, `repr` e alias global. Módulos testam kind,
inventário fechado e acesso a todos os filhos.

Metadata é observável: nome, `self`, ordem, positional/named, required,
variadic, settable e default devem coincidir com o residual pinado. Uma função
que apenas existe, mas cuja chamada real diverge, permanece aberta.

`pdf.artifact` mantém o contrato L0: `kind:` aceita os 12 valores medidos e o
body passa visualmente; tagging permanece scope-out. `pdf.attach` preserva a
metadata vanilla, mas a chamada continua no erro explícito de embedding
formalizado no L0; esse erro não é promovido a paridade funcional de embedding.

## 7. Decisão ADR-0127

### Fluxo contínuo autorizado, sempre com L0 proprietário primeiro

- Projeções não ligadas de métodos existentes por tabelas/wrappers privados.
- Funções estáticas e constantes construídas com `Value`, `Type`, `Func`,
  `Dir`, `Alignment`, `Length`, `Color`, `Args`, `Duration`, `Selector`,
  `State` e `Location` já existentes.
- `Type::Arguments` chamável por dispatch fechado existente, construindo
  `Value::Args` e preservando posicionais, named, ordem, valores e span.
- `cmyk` com quatro componentes Ratio no owner `foundations/color`, com as
  formas global e `color.cmyk` delegando à mesma nativa.
- `hsl`/`hsv` com `Angle` + dois `Component`, e `oklab`/`oklch` com a
  combinação exata de `RatioComponent`, `ChromaComponent` e `Angle` fixada no
  L0; alpha e aliases qualificados preservam os mesmos casts/defaults.
- Aliases qualificados que reutilizam exatamente o binding/tabela existente.
- `color.map` como `Value::Module` e seus 15 arrays de cores: nenhuma entidade,
  variante `Value`, `Type` ou `Content` nova é necessária.
- `selector.and`, `selector.or` e `selector.within`, exclusivamente sobre as
  variantes públicas já existentes. `before` e `after` não pertencem a este
  lote.
- Observação/testes de metadata e correções internas de tabela que não mudem
  contrato Rust público, default efetivo, fase ou compatibilidade.
- Descarga dos 15 `UNKNOWN` de `color.map.*` somente depois de o ancestral
  existir e cada valor ser comparado; antes disso continuam `Unknown`.

O fluxo contínuo autoriza a superfície de linguagem, não a expansão da API
Rust por conveniência. A auditoria pré-candidato encontrou representação
pública suficiente para os restantes itens autorizados: coleções, strings,
bytes, argumentos, duração, comprimento, direção, alinhamento, estado,
localização, `color.map` e metadata PDF podem ser implementados por wrappers,
tabelas ou módulos privados sobre `Value`/`Type`/entidades existentes.
`arguments.len` deve contar posicionais e named na linguagem sem alterar
silenciosamente a semântica do método Rust público `Args::len`; se uma solução
propuser essa mudança pública, ela sai do fluxo contínuo e exige novo gate.

Resultado da auditoria de contrato público:

| Superfície | Representação pré-candidata | Decisão |
|---|---|---|
| coleções, `str`, `bytes`, `arguments` | `Value::{Array,Dict,Str,Bytes,Args}` e campos existentes | contínuo por wrapper privado |
| direção, alinhamento, duração, comprimento | entidades/variantes já existentes | contínuo por wrapper/alias privado |
| `selector.and/or/within` | `Selector::{And,Or,Within}` | contínuo |
| `selector.before/after` | nenhuma variante `Before`/`After` | `BLOCKED_ADR0127_PUBLIC_CONTRACT` |
| estado e localização | entidades e variantes já existentes | contínuo por dispatch privado |
| `color.map` | `Value::Module`, `Value::Array`, `Color` existente | contínuo, sem entidade nova |
| `color.spot/tint` | falta tipo/variante spot | `BLOCKED_ADR0127_PUBLIC_CONTRACT` |
| `outline.entry/*` | falta elemento/`Content` correspondente | `BLOCKED_ADR0127_PUBLIC_CONTRACT` |
| metadata PDF já formalizada | `Func`/`Module` existentes | contínuo; embedding/tagging não autorizado |

Não foi encontrada outra obrigação do lote contínuo que exija, por necessidade
semântica, variante, campo, método de trait ou assinatura Rust pública nova.
Uma implementação que escolha introduzi-los não herda autorização deste selo.

### Bloqueios obrigatórios; este selo não autoriza implementação

1. **`color.spot` / `color.spot.tint`:** o vanilla possui `SpotColorant`,
   `SpotColor` e a variante `Color::Spot`. Os L0 atuais de `entities/color` e
   `stdlib/color` não modelam nem autorizam essa superfície. Materializá-la
   exige novo tipo/representação pública e revisão de matches/consumers;
   portanto é `BLOCKED_ADR0127_PUBLIC_CONTRACT`. Não existe decisão vigente
   que permita classificá-la como divergência intencional. `spot.tint` continua
   `Unknown(blocked_by_ancestor)` até decisão humana e L0 atualizado.
2. **`outline.entry` e cinco subfields:** o vanilla usa `OutlineEntry` como
   elemento/função com scope próprio. Os L0 atuais modelam somente
   `OutlineElem`; não autorizam novo `Content`, elemento, kind ou transporte.
   Estado: `BLOCKED_ADR0127_PUBLIC_CONTRACT`. Os subfields permanecem
   `Unknown(blocked_by_ancestor)`; não são sucesso nem divergência intencional.
3. **`selector.before` / `selector.after`:** o enum público cristalino possui
   `And`, `Or` e `Within`, mas não `Before`/`After`; há consumers com matches
   que precisam de revisão coordenada. Materializar os métodos exige variantes
   públicas e mudança do contrato de `Selector`, logo ambos são
   `BLOCKED_ADR0127_PUBLIC_CONTRACT`. O L0 vigente cita as formas vanilla como
   medição, mas não as materializa nem autoriza. Ausência é `Blocked`; mapear
   `before`/`after` para `Within`, `And`, `Or` ou composição aproximada é
   violação semântica, não divergência intencional.
4. **Metadata/defaults de `outline`:** o L0 vigente especifica defaults e
   domínio diferentes do catálogo residual (`depth: 3`/targets cristalinos
   versus metadata vanilla `depth:none`, `target:heading`). Testes podem medir,
   mas qualquer mudança de default/comportamento exige paragem humana e novo
   selo conforme ADR-0127.
5. Qualquer novo campo público, variante de enum público, método em trait,
   assinatura Rust pública, default de produto, fase eval/layout, remoção de
   extensão ou quebra de compatibilidade exige o mesmo gate.
6. Alterar embedding/tagging de PDF, em vez de manter os scope-outs L0, é eixo
   de produto/pipeline separado e não está autorizado por P1284.

## 8. Política de `Unknown`

`Unknown` só é permitido para ancestral ausente, identidade opaca, parser sem
suporte, pin divergente ou impossibilidade documentada de observação. Nunca é
convertido implicitamente em `Preserved`.

- Os 15 filhos de `color.map` são Unknown transitório e precisam ser
  reclassificados individualmente depois da materialização autorizada do módulo.
- `color.spot.tint` e os cinco filhos de `outline.entry` permanecem Unknown
  enquanto os respectivos gates humanos estiverem fechados.
- `selector.before` e `selector.after` não são `Unknown`: o requisito vanilla e
  a ausência das variantes estão determinados, portanto seu estado é
  `Blocked(BLOCKED_ADR0127_PUBLIC_CONTRACT)` até novo L0 e decisão humana.
- Um ancestral bloqueado não autoriza fabricar stubs, `none`, dicionários ou
  passthroughs para fazer os filhos parecerem presentes.

## 9. Matriz mínima de mutações

Mutações negativas válidas para o subconjunto autorizado:

1. omitir uma projeção não ligada;
2. expor método de instância como constante;
3. expor constante como função;
4. trocar função estática por método com `self`;
5. remover ou deslocar `self` na assinatura;
6. trocar positional por named ou vice-versa;
7. trocar required por optional;
8. alterar um default;
9. remover variadic ou aceitar argumentos além do contrato;
10. aceitar named desconhecido;
11. devolver kind correto com comportamento de outra função;
12. fazer forma ligada e não ligada divergirem;
13. quebrar curto-circuito/ordem observável de callback;
14. alterar erro ou classe/span diagnóstico observável;
15. mutar valor ou `repr` de direção/alinhamento;
16. duplicar alias em tabela divergente do global;
17. expor `color.map` como dict/type em vez de module;
18. omitir um dos 15 presets;
19. alterar ordem, tamanho ou uma cor de um preset;
20. contar só posicionais em `arguments.len`;
21. remover named ao mapear/filtrar `arguments`;
22. confundir bytes, chars e grapheme clusters;
23. executar método contextual sem contexto ou ignorar Location;
24. trocar conversão/unidade de `length`/`duration`;
25. alterar sem gate um default de `outline`;
26. fazer `pdf.attach` simular sucesso ou perder sua metadata;
27. promover um `Unknown` a sucesso por default;
28. criar segundo consumer para um L0 ou segundo L0 para um consumer;
29. expor `selector.before/after` sem gate, ou simulá-los com
    `Within`/`And`/`Or`/composição aproximada;
30. adicionar campo, variante, método de trait ou assinatura Rust pública para
    um item cujo contrato pode ser satisfeito por wrapper/tabela privada;
31. deixar `Type::Arguments` não chamável, rejeitar/perder/reordenar named ou
    reconstruir um `Args` semanticamente diferente da entrada variádica;
32. rejeitar Ratio válido em `cmyk`, aceitar Float/Int no seu lugar, permutar a
    ordem CMYK, omitir a validação 0%–100% ou divergir entre a forma global e
    `color.cmyk`;
33. tratar hue HSL/HSV como número sem unidade, trocar `Component` por Float,
    rejeitar Int/Ratio válido, aceitar valor fora dos ranges ou alterar alpha
    default 100%;
34. trocar lightness/alpha Oklab/Oklch de `RatioComponent` para Int/Float,
    trocar/permutar `ChromaComponent` e `Angle`, escalar Ratio chroma por fator
    diferente de 0,4 ou adicionar clamp inexistente;
35. fazer qualquer forma qualificada `color.hsl/hsv/oklab/oklch` divergir da
    nativa global correspondente em cast, ordem, valor, erro ou default.

Controles de gate, fora do denominador:

1. `color.spot` ausente antes da decisão humana -> `Blocked`, não `Violated`;
2. implementação de `color.spot/tint` sem novo L0/gate -> `Violated`;
3. `outline.entry` ausente antes da decisão humana -> `Blocked`;
4. stub de `outline.entry` sem nova entidade/semântica -> `Violated`;
5. filho de ancestral ausente -> `Unknown`, nunca `Preserved`;
6. `color.map.*` ainda Unknown depois do módulo e sem probe individual ->
   contrato não fechado;
7. `selector.before/after` ausentes antes do novo L0/gate -> `Blocked`, não
   `Violated` nem `Unknown`;
8. implementação de `selector.before/after` sem variantes públicas autorizadas
   e revisão dos consumers -> `Violated`;
9. alias/composição aproximada de `selector.before/after` -> `Violated`;
10. `selector.and/or/within` sobre as variantes existentes, com chamadas ligada
    e não ligada equivalentes -> `Preserved` somente após RED->GREEN e probes.

Gate discriminatório exigido antes do certificado:
`mutation_score = mutações negativas válidas rejeitadas / mutações negativas
válidas = 1.0`, com os controles recebendo exatamente as classificações acima.

## 10. Fechamento e capacidades

Para cada lote/família: L0 atualizado e resselo; RED por resolução e chamada
real; implementação; inventário default/html regenerado; metadata bilateral;
testes do owner; build; suíte completa; V15/V26; `crystalline-lint .`.
Nenhum percentual global substitui o fechamento dos 180 paths ou a decisão
formal dos bloqueados.

Capacidades desta autoridade:

- leitura allowlisted de `AGENTS.md`, passo autorizado, residual P1283, L0s das
  famílias, fontes vanilla ratificadas e recibos/oráculos já selados;
- escrita allowlisted neste resselo somente deste diagnóstico; nenhum L0 foi
  editado;
- sem leitura de código, testes, patches ou diffs candidatos P1284;
- sem edição de código produtivo, testes ou oráculos;
- sem execução de veredito contra implementação.

Limitação: workspace e contexto são compartilhados. A independência é
procedimental, apoiada por allowlist, hashes e ordem causal, mas não é
atestação de isolamento técnico por sandbox/worktree independente. Mudança em
qualquer entrada congelada invalida `SEALED_SPEC_FOR_IMPLEMENTATION`.

Os recibos de oráculo anteriores permanecem evidência histórica da medição
vanilla, mas qualquer selo que referencie C-v6 ou pins L0 predecessores deve
ser repinado contra este v7 antes do certificado final. Esse repin não autoriza
alterar a candidata congelada.
