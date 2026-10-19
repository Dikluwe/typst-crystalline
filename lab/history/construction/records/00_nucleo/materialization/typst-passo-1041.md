# Passo 1041 — Refactor V16: 8 DENY, neutros nomeados, FrameVisitor

**Dependência formal**: `ADR-0016` Rev. 1 (tekt-linter) está **fechado como ACEITO** —
confirmado pelo dono. Sem reserva pendente.
**Precede este passo**: saga de medição de decisões (2026-08): varredura `syn` R1–R7 →
`ADR-0016` → implementação V16–V20 (Passo 0063, tekt-linter) → calibração (Passo 0064) →
reconciliação (Passo 0065). Os números abaixo (8 / 132 / 43) são os do Laudo 0064,
ratificados pelo fecho do ADR-0016 Rev. 1.
**Pré-condição**: `git status` limpo em `typst-crystalline`.

---

## Princípio reitor

Cada alteração **tem de ser confirmada como comportamentalmente neutra**, não presumida
como tal. "Neutra para o input actual, diferente para variantes futuras" é o resultado
esperado da maior parte dos 195 casos — mas não é uma suposição a aplicar em bloco. Cada
caso onde a Fase B ou C revelar que o comportamento actual **já estava errado** (o próprio
passo prevê isto — ex.: `is_opaque` incorrecto para um espaço de cor concreto) deixa de
ser neutro nesse ponto específico, e **activa `ADR-0127`** — paragem obrigatória, L0 e
confirmação do dono antes de corrigir esse caso, mesmo que o resto do passo prossiga em
fluxo contínuo. Não presumir de antemão quantos casos serão esse tipo — contar e reportar,
não estimar.

Nenhum dos 8 DENY se resolve com "manter o default e suprimir o lint": a decisão semântica
de cada caso é o *deliverable*.

---

## Fase A — Os 8 DENY (saturação arbitrária)

Para cada caso (lista final do Laudo 0064/rev. 0065; exemplos conhecidos: `Unit::Percent`,
`Assoc::Left`, `ULTRA_EXPANDED`, `1.0` em `MathStyle`):

1. Substituir `_ => <default>` pelos braços nominais das variantes restantes.
2. Para cada variante que herdaria o default, decidir e registar no relatório, **com
   evidência, não suposição**:
   - **(a)** o default era semanticamente correcto → braço explícito com comentário
     "intencional: `<razão>`" — a razão tem de ser verificável (citação de comportamento
     vanilla, invariante do domínio, ou medição), não "parece razoável".
   - **(b)** merecia valor próprio → implementar. **Isto muda comportamento observável —
     gate `ADR-0127` antes de codificar**, não depois.
   - **(c)** devia falhar → erro tipado. Mesma exigência de gate se isto muda o que hoje
     compila silenciosamente para o que passa a rejeitar.
3. **Proibido resolver por `[wildcard_exceptions]` nesta fase** — excepções são para hubs
   intencionais (Fase C, cores), não para saturação. Se a Fase A revelar que algum "DENY"
   era afinal um hub intencional, não forçar exaustividade — mover para
   `[wildcard_exceptions]` com justificativa real e registar a reclassificação, não
   silenciar.

**Conclusão da Fase A**: 0 DENY; tabela das 8 decisões no relatório, cada uma marcada
(a)/(b)/(c) com a evidência; invariante novo — adicionar variante a
`Unit`/`BinOp`/`FontStretch`/`MathStyle`/`Color`/`Func` passa a falhar a compilação nos
pontos de decisão (prova por mutação fantasma, revertida no fim, per Fase C).

## Fase B — Os 132 defaults neutros

Conversão com decisão por caso, não mecânica em bloco: `_ => false` passa a `other =>
false // neutro: <razão de uma linha>` (ou braços nominais quando a variante concreta for
mais legível).

**Qualquer caso onde a revisão revelar que o neutro estava errado** (ex.: `is_opaque` para
um espaço de cor concreto) é um bug encontrado por este passo — **não corrigir inline
presumindo que é seguro**. Registar como achado, aplicar `ADR-0127`: se a correcção muda
output observável (mesmo que pareça pequena), L0 e confirmação antes de codificar essa
linha específica. As restantes 131 (ou o número que sobrar) seguem em fluxo contínuo, sem
esperar pela confirmação da excepção.

**Conclusão da Fase B**: 0 wildcards anónimos não-erro em enums de domínio; cada neutro
com justificativa de uma linha verificável; bugs encontrados listados explicitamente —
**"nenhum" é resultado válido, mas tem de ser afirmado, não inferido por omissão**.

## Fase C — `FrameVisitor` (os 43 walkers)

1. Criar `trait FrameVisitor` (localização a decidir: `03_infra/src/`, junto dos walkers,
   respeitando as camadas — confirmar por leitura onde as fronteiras L1-L4 permitem isto
   antes de escolher, não presumir `03_infra` só porque é onde os walkers vivem hoje).
2. `visit_item` recursivo sobre `Group`/`Link`, hooks `visit_text`/`visit_image`/
   `visit_shape` com default vazio, e ponto único de fallback:
   ```rust
   _ => { debug_assert!(false, "FrameItem não tratado: {:?}", item); }
   ```
   (no-op em release, falha ruidosa em debug/testes — comportamento **idêntico** ao actual
   nos 43 sítios; confirmar isto por leitura de cada um antes de assumir que todos têm o
   mesmo padrão de fallback hoje).
3. Migrar os 43 walkers de `export/fonts.rs`, `export/images.rs`, `export/builder.rs`,
   `shaper.rs`, `pipeline.rs` para impls do visitor.
4. **Teste obrigatório**: adicionar variante fantasma a `FrameItem` (branch descartável),
   confirmar que o `debug_assert!` dispara na suite, **depois reverter** — a prova fica no
   relatório (output do teste antes de reverter), não só a afirmação de que foi feita.

**Conclusão da Fase C**: 43 `_ => {}` dispersos colapsam num único ponto auditado;
`cargo test` verde; **diff de comportamento vazio** — confirmar com os testes de
referência/snapshots existentes, não presumir "deve ser igual" só pela estrutura do
refactor.

## Fase D — Ratchet

`crystalline.toml` do `typst-crystalline`: `V16 = { level = "error", languages = ["rust"]
}`. `V17`/`V18` mantidos `warning` até worklist própria (29 guards + 2 ranges) ser atacada
num passo posterior — **não** promover a `error` neste passo, âmbito é só V16. CI passa a
bloquear regressões de V16 a partir daqui.

---

## Critérios de aceitação

- [ ] (A.1) 0 violações V16 DENY-class; 8 decisões documentadas, cada uma com evidência
      (a)/(b)/(c), não suposição
- [ ] (A.2) 132 neutros nomeados/justificados; bugs encontrados listados explicitamente
      (zero incluído como resultado válido, se for o caso)
- [ ] (A.3) `FrameVisitor` adoptado pelos 43 walkers; mutação fantasma prova o fallback
      (output do teste no relatório) e é revertida
- [ ] (A.4) V16 = `error` no config; `crystalline-lint --checks v16,v17,v18 .` sem DENY
- [ ] Todo bug/correcção não-neutra encontrado nas Fases A/B/C passou por gate `ADR-0127`
      explícito antes de codificado — listado no relatório com o gate aplicado, não
      implementado silenciosamente
- [ ] `cargo build --workspace --release` ok; `cargo test` verde; nenhum snapshot de
      output alterado fora dos casos com gate explicitamente aprovado

## Comandos de validação

```bash
cargo build --workspace --release
cargo test --workspace
crystalline-lint --checks v16,v17,v18,v19,v20 .
```

## Esqueleto do relatório

`typst-passo-1041-relatorio.md`: Resumo executivo (8/132/43 → 0 DENY, V16 em `error`) →
Metodologia → Fase A (tabela das 8 decisões, cada uma com evidência e classe a/b/c) →
Fase B (neutros; bugs encontrados, com gate `ADR-0127` aplicado onde relevante) → Fase C
(`FrameVisitor`; prova da mutação fantasma revertida, com output do teste) → Fase D
(ratchet) → estado da árvore → Proveniência.

## Notas para o executor

Fases commitáveis em separado (A, depois B, depois C, depois D) — se algo correr mal a
meio, o repo fica num estado válido em qualquer fronteira de fase. **Não avançar de fase
com uma correcção não-neutra pendente de gate** — parar nessa fase, reportar, esperar
confirmação, só depois continuar para a fase seguinte.
