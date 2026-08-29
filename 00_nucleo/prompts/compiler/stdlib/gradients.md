# Prompt L0 — `stdlib/gradients` — tipo `gradient`
Hash do Código: ab29e4ac

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/gradients.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P262 (`gradient.linear`), P264 (`gradient.radial`) e P267 (`gradient.conic`).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded), ADR-0087
(Gradient Linear-only, posteriormente expandido), ADR-0091 (ColorSpace runtime).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.
**Entidade subjacente**: ver `00_nucleo/prompts/entities/gradient.md`.

---

## Tipo `gradient` — representação no scope (P736)

**P736 — estado vigente:** `gradient` é exposto no scope global como
`Value::Type(Type::Gradient)` (paridade vanilla — medido: `type(gradient)`
→ `type`; `type(gradient.linear(red, blue)) == gradient` → `true`;
`repr(gradient)` → `gradient`; `gradient(...)` → erro "type gradient does
not have a constructor"). Histórico: até P736 era `Value::Dict`
(`make_gradient_module()`).

Os fields `linear`/`radial`/`conic` resolvem-se por field access em
`Value::Type` via `gradient_type_field(field) -> Option<Value>` (padrão
P685); campo inexistente → erro "type gradient does not contain field
`<f>`" (mensagem verbatim do vanilla, medida).

### Formato de stops

Argumentos posicionais variádicos (`args.items`). Cada stop aceita:

- `Value::Color(c)` → `GradientStop::unspaced(c)` (offset automático).
- `Value::Array([Color, Ratio])` → `GradientStop::new(c, offset)`.
- `Value::Array([Color, Float/Int])` → o número é convertido para `Ratio`.

Pelo menos um stop é obrigatório; zero stops → erro.

### ColorSpace

O named argument `space` aceita `"oklab"`, `"oklch"`, `"srgb"`, `"luma"`,
`"linear-rgb"`, `"hsl"`, `"hsv"`, `"cmyk"`. Default `"oklab"` (paridade vanilla).
A interpolação usa o dispatcher `interpolate_in_space` definido em
`entities/gradient.rs`.

**P1252 — normalização Luma com alpha preservado.** Os constructors Linear,
Radial e Conic convertem cada stop para o mixing space por `Color::to_space`.
Quando `space: luma`, a normalização deve preservar o alpha da origem nos três
variants. O vanilla ratificado perde esse alpha para stops não-Luma; o
cristalino classifica a preservação como `Known-Upstream-Bug`. Esta escolha não
fecha o delta de luminância e não autoriza promoção SVG.

### `relative`

Named argument `relative` aceita `"self"`, `"parent"` ou `"auto"`. Default
`"auto"` (resolvido como `Self_`). Representação interna `Option<RelativeTo>`.

---

### `native_gradient_linear` (`gradient.linear`)

**Assinatura vanilla**:
```typc
gradient.linear(..stops, angle: angle, space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `angle`: `Angle` ou `Float` (radianos). Default `0deg`.
- `space`: string de color space (ver acima). Default `"oklab"`.
- `relative`: `"self" | "parent" | "auto"`. Default `"auto"`.

**Semântica**: Constrói `Value::Gradient(Gradient::Linear(Linear { stops, angle, space, relative }))`.

**Paridade vanilla**: Equivalente a `#gradient.linear(red, blue, angle: 45deg)`.

**Limitações / scope-outs**:
- PDF render não desenha gradiente real; `Paint::to_color()` faz fallback para a
  cor do primeiro stop (`first_stop_color()`).
- `tiling(gradient)` é rejeitado em `visualize.rs` (scope-out ADR-0054).
- `anti_alias` não exposto.

**Testes canônicos**:
```
gradient.linear(red) -> Value::Gradient(Linear)
gradient.linear(red, blue, angle: 45deg) -> Linear com angle = 45deg
gradient.linear() -> Err "pelo menos 1 stop requerido"
gradient.linear(red, angle: 90deg, space: "srgb") -> space = Srgb
gradient.linear(red, relative: "parent") -> relative = Some(Parent)
gradient.linear(red, foo: 1) -> Err "argumento nomeado inesperado"
```

---

### `native_gradient_radial` (`gradient.radial`)

**Assinatura vanilla**:
```typc
gradient.radial(..stops, center: (50%, 50%), radius: 50%,
                focal-center: center, focal-radius: 0%,
                space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `center`: array `[Ratio, Ratio]` ou `[Float/Int, Float/Int]`. Default `(50%, 50%)`.
- `radius`: `Ratio`, `Float` ou `Int`. Default `50%`. Deve estar em `[0, 1]`.
- `focal_center`: mesmo formato de `center`. Default igual a `center`.
- `focal_radius`: mesmo formato de `radius`. Default `0%`.
- `space` / `relative`: idem `linear`.

**Semântica**: Constrói `Value::Gradient(Gradient::Radial(Radial { stops, center, radius, focal_center, focal_radius, space, relative }))`.

**Validações**:
- `focal_radius > radius` → erro.
- Distância de `focal_center` a `center` deve ser `< radius - focal_radius`
  (focal circle dentro do outer circle); caso contrário → erro.

**Paridade vanilla**: Equivalente a `#gradient.radial(red, blue)`.

**Limitações / scope-outs**:
- PDF render fallback para cor do primeiro stop.
- `tiling(gradient)` rejeitado.

**Testes canônicos**:
```
gradient.radial(red) -> Value::Gradient(Radial)
gradient.radial(red, blue, radius: 30%) -> radius = 0.3
gradient.radial(red, focal-radius: 10%) -> focal_radius = 0.1, focal_center = center
gradient.radial(red, focal-radius: 60%) -> Err "focal_radius > radius"
gradient.radial(red, center: (0%, 0%), focal-center: (80%, 80%), radius: 50%) -> Err "focal circle fora"
gradient.radial() -> Err "pelo menos 1 stop requerido"
```

---

### `native_gradient_conic` (`gradient.conic`)

**Assinatura vanilla**:
```typc
gradient.conic(..stops, center: (50%, 50%), angle: 0deg,
               space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `center`: array `[Ratio, Ratio]`. Default `(50%, 50%)`.
- `angle`: `Angle` ou `Float` (radianos). Default `0deg`.
- `space` / `relative`: idem `linear`.

**Semântica**: Constrói `Value::Gradient(Gradient::Conic(Conic { stops, center, angle, space, relative }))`.

**Paridade vanilla**: Equivalente a `#gradient.conic(red, blue, angle: 90deg)`.

**Limitações / scope-outs**:
- Sem `focal_*` (não existem em ConicGradient vanilla).
- PDF render fallback para cor do primeiro stop.
- `tiling(gradient)` rejeitado.

**Testes canônicos**:
```
gradient.conic(red) -> Value::Gradient(Conic)
gradient.conic(red, blue, angle: 90deg) -> angle = 90deg
gradient.conic(red, center: (25%, 75%)) -> center = (0.25, 0.75)
gradient.conic() -> Err "pelo menos 1 stop requerido"
gradient.conic(red, focal-radius: 5%) -> Err "argumento nomeado inesperado"
```

---

## Fronteira com render PDF

As funções nativas produzem `Value::Gradient`; a entidade implementa as três
variantes, interpolação multi-space e auto-spacing. O render PDF vigente é
owner de L3 e já possui shading dedicado. P1144 não altera nem reespecifica
esse pipeline: audita somente a superfície de linguagem em eval/stdlib.

---

## P1144 — família pública de métodos de `gradient`

### Medição anterior à decisão

Fonte ratificada `a51e02804`,
`crates/typst-library/src/visualize/gradient.rs:719-874`: o scope público tem
onze métodos, `kind`, `stops`, `space`, `relative`, `angle`, `center`,
`radius`, `focal-center`, `focal-radius`, `sample` e `samples`. Sondas nos dois
binários vanilla ratificados confirmaram a mesma superfície estática
(`gradient.kind(g)`) e de instância (`g.kind()`), com resultados iguais.

Isto é semântica e morfologia da linguagem (ADR-0107), não uma exigência sobre
a representação Rust. A entidade vigente já contém todos os campos, offsets
efetivos e funções de sampling necessários; P1144 adiciona apenas glue interno.
Não altera contrato público Rust, default, compatibilidade nem fase do
pipeline, portanto segue fluxo contínuo segundo ADR-0127: L0 primeiro,
resselo, RED→GREEN e revalidação.

### Fields estáticos e métodos de instância

`gradient_type_field` expõe os onze nomes como `Func`, além dos constructors.
Os constructors têm nomes funcionais `linear`, `radial` e `conic`, de modo que
`repr(g.kind())` preserve a morfologia vanilla e a identidade com o field.
As formas estática e de instância delegam à mesma implementação; a forma de
instância sintetiza o gradiente como primeiro positional. Argumentos nomeados
são rejeitados. Accessors recebem apenas `self`; `sample` recebe exatamente um
ratio ou angle; `samples` recebe zero ou mais ratios/angles posicionais.

### Retornos por variante

- `kind` devolve a própria função `gradient.linear`, `.radial` ou `.conic`.
- `stops` devolve array de pares `[Color, Ratio]`, sempre com offsets efetivos
  resolvidos, preservando o espaço de cor dos stops.
- `space` devolve a função construtora do espaço: `rgb`, `linear-rgb`, `luma`,
  `cmyk`, `hsl`, `hsv`, `oklab` ou `oklch`.
- `relative` devolve `auto` para `None`, e as strings `"self"` ou `"parent"`
  para os valores explícitos.
- `angle`: Linear/Conic → `Angle`; Radial → `none`.
- `center`: Radial/Conic → `[Ratio, Ratio]`; Linear → `none`.
- `radius`, `focal-center`, `focal-radius`: Radial → o campo correspondente;
  Linear/Conic → `none`.
- `sample(t)` converte ratio diretamente e angle para fração de volta
  (`radianos / 2π`), delega no sampling da variante e preserva clamp `[0, 1]`.
- **P1253:** no Linear, essa delegação preserva o `f64` público até o cálculo
  do peso local; conversão antecipada para `f32` é proibida porque altera os
  componentes observáveis nos pontos exatos da malha.
- **P1269-owner:** no Linear, a razão efetiva já exposta por `stops()` é uma
  identidade pública de fronteira: quando `sample` a recebe exatamente, ela é
  normalizada para o offset canônico `Ratio(f64)` correspondente antes do
  sampling preciso. Em stops coincidentes, isso seleciona o primeiro stop
  daquele offset, exceto no offset zero, que seleciona o último stop zero;
  qualquer epsilon positivo seleciona o ramo à direita. Depois de P1271-C3b,
  `stops()` expõe o carrier preciso; a compatibilidade com identidades `f32`
  históricas permanece, sem autorizar arredondar argumentos gerais de
  `sample`, e vale igualmente para `samples`.
- `samples(..ts)` aplica `sample` a cada posição, preservando ordem; zero
  posições devolve array vazio.

**P1271-C4 — sampling Radial preciso.** Medição em `e055a12f` localizou o
estreitamento `Ratio(f64) → f32` no braço Radial de `sample_gradient`; em
`37.123456789%` ele alterava componentes públicos Oklab e Linear RGB. O braço
Radial deve encaminhar `f64` ao sampling preciso da entidade, preservando clamp
e as formas `sample`/`samples`. Linear permanece no caminho preciso vigente;
Conic e demais espaços são scope-out. Nenhuma assinatura pública, default,
fase, budget ou caminho de render muda.

### Verificação P1144

- cobrir os onze fields estáticos e as onze formas de instância;
- cobrir a matriz de ausências por variante e identidade de `kind`/`space`;
- cobrir offsets automáticos resolvidos, ratio, angle, clamp e lista vazia;
- cobrir coincidências Linear não diádicas pela razão devolvida por `stops()`,
  incluindo `1/3`, `2/3` e epsilon à direita em Oklab e Linear RGB;
- cobrir self errado, falta/excesso, named arg e posição de tipo inválido;
- manter constructors e render existentes sem regressão.

---

## P1145 — constructors, `sharp` e `repeat`

### Medição ratificada anterior à decisão

No objeto `a51e02804`, `visualize/gradient.rs:243-875`, o scope público
completo contém dezasseis funções: três constructors (`linear`, `radial`,
`conic`), duas transformações (`sharp`, `repeat`) e os onze membros auditados
em P1144. Assim, “11/11” em P1144 significa somente o subconjunto então
inventariado, não o scope completo.

Os dois binários vanilla ratificados coincidiram nas sondas positivas e em
quatorze negativas. Contratos medidos:

- todos os constructors exigem pelo menos dois stops;
- se um stop tem offset, todos devem ter; offsets são monotônicos, pertencem a
  `[0%, 100%]`, começam em `0%` e terminam em `100%`;
- `linear` aceita `dir:` ou `angle:`; com ambos, `dir` sobra e é argumento
  inesperado; `ltr/rtl/ttb/btt` resolvem para `0/180/90/270deg`;
- Radial usa os nomes públicos `focal-center:` e `focal-radius:`; underscore é
  argumento inesperado;
- focal circle tangente ou exterior é inválido;
- `sharp(steps, smoothness:)` exige `steps >= 2` e smoothness em `[0%,100%]`;
- `repeat(repetitions, mirror:)` exige repetitions >= 1;
- ambas as transformações preservam variant, geometria, space e relative.

### Correções internas independentes

Após o gate, o owner deve alinhar parser/validação sem alterar contratos Rust:

- mínimo de dois stops e regras integrais de offsets;
- nomes focais com hífen; compatibilidade underscore só pode permanecer se
  decisão explícita a classificar como extra compatível;
- `dir` no Linear com precedência/exclusão medida;
- mensagens e hints observáveis;
- fields estáticos e despacho de instância para `sharp` e `repeat`, delegando
  às mesmas funções privadas do owner.

### Transformações propostas

`sharp(self, steps, smoothness: 0%) -> gradient` constrói `2 * steps` cores e
offsets conforme a fórmula ratificada, deduplica stops coincidentes, preserva
variant/fields e define `anti_alias = false`.

`repeat(self, repetitions, mirror: false) -> gradient` replica e reescala os
stops para cada subintervalo, inverte as repetições ímpares quando `mirror` é
true, deduplica fronteiras, preserva variant/fields e preserva `anti_alias`.

**P1271-C3 — precisão de `repeat`.** Medição em `e055a12f` mostrou que quatro
stops automáticos repetidos duas vezes expunham `1/6`, `1/3`, `2/3` e `5/6`
arredondados primeiro para `f32`, ao contrário do vanilla ratificado. Antes da
decisão, `resolved_stops` foi localizado como o estreitamento comum às três
variantes. A transformação deve consumir a resolução efetiva `f64` do owner de
entidade e reconstruir Ratios sem carrier `f32` intermediário. Não muda
assinaturas, validação, deduplicação, mirror, `sharp`, sampling ou render.
Aceitação: `stops()` da linguagem após `repeat(2)` coincide com os Ratios
vanilla nos três variants; a estrutura do helper Rust não é observável.

**P1271-C3b — estreitamento de saída medido após C3a.** Com `repeat` já
consumindo `f64`, o teste RED ainda expôs os mesmos offsets `f32`; a medição em
`native_gradient_stops` mostrou uma segunda chamada ao resolver histórico
`Vec<f32>`. O accessor de linguagem passa a usar a resolução precisa do owner
para Linear, Radial e Conic. O método Rust público `effective_offsets()` não
muda. Isto também alinha stops explícitos e automáticos comuns ao carrier
`Ratio(f64)` da linguagem; não altera `sharp`, sampling nem render.

As formas estática e de instância são públicas na linguagem. Sampling dos
stops transformados é critério primário; passos e alocação Rust são mecânica.

### Gate

Materializar `sharp` com paridade de render requereu o novo campo público Rust
`anti_alias` nas três structs de entidade. Isso é mudança de contrato público
literal e acionou ADR-0127. O dono aprovou o gate com “Continuar” antes da
alteração de código.

### Materialização aprovada

- os três constructors exigem dois stops e aplicam as regras completas de
  offsets;
- stops são convertidos para o mixing space; `space: auto` resolve Oklab;
- Linear aceita `dir`; Radial usa os nomes focais com hífen;
- `sharp` e `repeat` estão nas formas estática e de instância;
- angles de sampling usam `rem_euclid(2π)` antes do clamp;
- `sharp` define `anti_alias = false`, `repeat` preserva e constructors usam
  `true`.
