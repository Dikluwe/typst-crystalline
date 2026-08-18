# L0 — Passo 1065: Anotação Formal dos 46 Casos `/2.0` (Classes 1A, 1B, 1C)

**Gate**: `ADR-0127` — não muda comportamento do compilador, mas altera 46 arquivos
de código de produção (comentários de silêncio) e reduz a superfície de violações
reportadas pelo `crystalline-lint`. **Requer confirmação do dono antes de executar
em massa**, mesmo mecanismo de cautela já usado no P1062 (renomeação de arquivos)
para mudanças que tocam muitos arquivos de uma vez.

**Base**: P1064 (auditoria, 14/14 casos da amostra confirmados com prova real —
citação literal, cadeia de chamadas, ou álgebra + medição). As 3 classes (1A
centragem, 24 casos; 1B semi-espessura, 11 casos; 1C ponto médio/eixo, 11 casos)
partilham a mesma justificação de isenção: divisão por 2 para achar metade/meio de
uma grandeza é auto-evidente pela própria álgebra, não uma decisão de design com
proveniência externa a citar.

---

## 1. Ressalva sobre 1C, antes de anotar

`height / 2.0 + axis` tem duas partes. O `/2.0` é tão auto-evidente quanto 1A/1B —
**mas o termo `axis` é um valor separado, com proveniência própria** (tipicamente
métrica de fonte — altura do eixo matemático). A anotação de isenção deste passo
cobre só o `/2.0`, não o termo `axis`.

Antes de anotar os 11 casos de 1C, confirmar que `axis` (ou o nome real da
variável em cada um dos 11 pontos) já tem a sua própria citação/proveniência
registada — noutro comentário, noutro passo, ou no próprio `vanilla_defaults.rs`
do P1058. Se não tiver, isso é um achado separado (constante sem proveniência,
mas na metade aditiva da expressão, não na divisão) — registar, não misturar com
a isenção do `/2.0`.

```bash
grep -B3 "axis" 01_core/src/compiler/math/layout/mod.rs | grep -i "provenien\|vanilla\|citação\|font"
```

(comando indicativo — confirmar contra os nomes reais de variável nos 11 pontos de
1C listados no P1064, não só `mod.rs`)

## 2. Mecanismo de anotação

Seguir a convenção já estabelecida para casos V21 legítimos nesta base de código
(mesmo padrão citado no P1064 para as constantes `0.65em`/`1.2em` com proveniência
válida — ver exemplo real de comentário `// neutro: ...` ou equivalente, a
confirmar contra um caso V21 já anotado no código antes de replicar o formato
exacto, não inventar sintaxe nova).

Proposta de texto (a validar contra a convenção real):

```rust
// neutro: /2.0 é geometria de centragem euclidiana universal — divide o
// espaço/grandeza ao meio, não uma decisão de design com proveniência
// externa a citar. Classe 1A (V21 Categoria 1, P1064).
```

Variantes equivalentes para 1B ("semi-espessura de traço") e 1C ("ponto médio;
nota: o termo `axis` somado tem proveniência própria, ver §1 — só o `/2.0` está
isento aqui").

## 3. Execução

- 24 casos de 1A + 11 de 1B + 11 de 1C = 46 arquivos/pontos a anotar, distribuídos
  pelos 14 arquivos já listados no inventário do P1064.
- Não reescrever nenhuma fórmula — só adicionar comentário. Zero mudança de
  comportamento.
- Depois de anotar, rodar `crystalline-lint --checks v21` e confirmar que as 46
  violações desaparecem do relatório (ou o número correcto, se algum caso de 1C
  ficar pendente por falta de proveniência do `axis`, per §1).

## 4. Critério de conclusão

- Resposta à ressalva do §1 para os 11 casos de 1C (proveniência de `axis`
  confirmada ou achado novo registado).
- 46 (ou 46 menos os pendentes de 1C) arquivos anotados.
- `crystalline-lint --checks v21` com a contagem de violações reduzida na medida
  esperada.
- `cargo test --workspace` — 100% pass (nenhuma mudança funcional esperada, mas
  confirmar mesmo assim).
- `crystalline-lint .` completo — 0 erros.

---

## Nota — a outra frente de "hardcode" continua disponível

Expansão dos módulos de constantes por domínio (`export/`, `stdlib/text/`,
seguindo o piloto do P1058) — não tocada por este passo, disponível a seguir.
