# Passo 962 — operador diferencial `d` (em `dx`/`dt`/etc.) deveria ser reto, não itálico

**Precede este passo**: auditoria externa, terceira rodada (2026-08-04) — `𝑑` itálico (cristalino)
vs `d` reto (vanilla) em `dx`/`dz`/`dt`/`dV`/`dS`, presente nas seções 4, 11, 12, 15, 17, 23, 25,
27. Sem mudança desde a primeira rodada.

**Contexto tipográfico (não presumir sem confirmar)**: a convenção de composição matemática usual
(ISO 80000-2 e a maioria dos motores modernos) trata o "d" do operador diferencial (`∫f(x) dx`)
como reto, distinto de uma variável chamada "d" (que seria itálica, como qualquer identificador de
1 letra). Isso não é o mesmo mecanismo do itálico automático de identificadores já corrigido em
P906/961 (Parte B) — é uma **exceção** a essa regra, específica para este símbolo neste contexto.

**Pré-condição de árvore**: `git status`. Confirmar P958-961 presentes.

---

## Fase A — confirmar o mecanismo

1. Confirmar como o `.typ` de teste produz o "d" nesses casos — usa o símbolo dedicado do Typst
   para diferencial (`dif`/`#sym.dif`, se existir) ou um "d" comum interpretado como identificador?
   Isto muda a causa completamente: se for `dif`, o cristalino pode não ter esse símbolo
   implementado (cai no caminho genérico de identificador, ganhando itálico por P906/961); se for
   "d" comum, a pergunta é diferente (por que o vanilla trata um "d" comum como reto neste
   contexto especificamente).
2. Ler o mecanismo real do vanilla (`lab/typst-original/`) para o símbolo/comportamento do "d"
   diferencial — confirmar exatamente como ele decide upright vs itálico neste caso.
3. Confirmar se isto é um símbolo dedicado (mapeamento de tabela, mesma classe de P895/902/958) ou
   uma regra contextual (detectar "d" seguido de variável de integração — mais complexo).

## Fase B — Implementação (TDD directo se for símbolo dedicado; protocolo de dois agentes se for
regra contextual nova)

1. Teste com os casos reais do documento (`dx`, `dt`, `dV`, `dS`) confirmando "d" reto.
2. Implementar conforme a causa confirmada.
3. Suíte completa verde, discriminada por crate. Confirmar que identificadores "d" genuínos
   (variável chamada d, não diferencial) continuam a receber itálico — não quebrar P906/961.
4. Confirmação visual/glifo.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

`compare.py` nas seções afetadas (4, 11, 12, 15, 17, 23, 25, 27). Benchmark completo, 7 cenários,
`depois/antes`.

## Resultado esperado

- Mecanismo confirmado (símbolo dedicado vs regra contextual).
- "d" diferencial reto nos casos reais, sem quebrar itálico de identificador "d" genuíno.
- Benchmark sem regressão.
