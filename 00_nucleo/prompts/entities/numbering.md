# Prompt L0 — `entities/numbering` — numbering tipado e vistas realizadas
Hash do Código: (pendente — P1158 materializa após gate ADR-0127)

**Camada:** L1
**Ficheiro alvo:** `01_core/src/entities/numbering.rs`
**Origem:** P1157 — auditoria de `page.numbering` por função.
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0127.

## 1. Medição anterior à decisão

No vanilla ratificado `a51e02804`, `model/numbering.rs:99-128` define
`Numbering::Pattern | Numbering::Func`. `layout/page.rs:290-311` documenta
que página visível chama função com número corrente e total, enquanto links e
referências chamam com apenas o corrente. `pages/run.rs:155-168` trata toda
função como dependente do total. `layout/introspect.rs:122-125` preserva o
objeto `Numbering` por página.

Sondas P1157, idênticas nos dois binários ratificados:

- callback `(current, total)` em duas páginas → `V1/2`, `V2/2`;
- callback variádico com footer explícito + `ref(form: "page")` → `R1`;
- `loc.page-numbering()` sobre callback → repr `(..) => ..`;
- counter lógico atualizado para 7 em três páginas → `N7/9`, `N8/9`,
  `N9/9`; referência à primeira → `N7`;
- page-runs lexicais callback/pattern → `X1/2`, `II`;
- função unária falha no número visível com `unexpected argument`; ternária
  falha com `missing argument: c`;
- retorno de função de numbering pode ser qualquer `Value`: inteiro 42 é
  exibido como `42`, `none` produz conteúdo vazio;
- footer explícito suprime o callback de dois argumentos na margem, mas a
  referência ainda o chama com um argumento (`AUTO1`).

## 2. Contrato

Adicionar entidade pública fechada:

```text
pub enum Numbering {
    Pattern(EcoString),
    Func(Func),
}
```

`Numbering` é dado de linguagem puro, clonável O(1) e sem I/O. A variante
`Pattern` preserva a string original; parsing/formatação permanece no owner
`compiler/stdlib/numbering`. A variante `Func` preserva a closure capturada.
Igualdade/hash seguem a identidade já definida por `Func`; não comparar
resultado renderizado nem bytes.

Estados de delta continuam distintos:

- campo exterior `None`: argumento omitido, preservar configuração ativa;
- `Some(None)`: `numbering: none`, desativar;
- `Some(Some(Numbering))`: instalar pattern ou função.

Snapshots de configuração/página usam `Option<Numbering>` porque já
representam o estado resolvido, não um delta.

## 3. Realização

Callback não pertence à entidade e nunca é executado aqui. O fixpoint com
Engine produz duas vistas por página:

- `visible`: callback com `[current, total]`;
- `reference`: callback com `[current]`.

O resultado de qualquer tipo é convertido pela semântica normal de conteúdo
da linguagem: `Content` permanece; `Str` vira texto; `None` vira vazio; demais
valores usam a mesma exibição em markup observada por `#numbering(callback)`.
Erros conservam o span da configuração de numbering.

## 4. Gate

Criar esta entidade e trocar campos públicos é mudança de contrato; P1157
para antes do código. P1158 só materializa após aprovação explícita.

## 5. Divisão explícita da materialização

P1158 materializou o enum, casts, deltas, snapshots e assinatura tipada do
Introspector. P1159 completou as vistas de callback e o ciclo layout → Engine →
relayout. `Numbering::Func` permanece cru no contrato, enquanto margem e
referência consomem conteúdo realizado com aridades binária e unária.
