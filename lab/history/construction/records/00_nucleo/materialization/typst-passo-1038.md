# Passo 1038 — Medir MC/DC real nos 5 nós fatiados + ponto de mensagens de erro

**Tipo**: Medição/diagnóstico — instrumentar e medir, não corrigir nem decidir nada ainda.
Responde à pergunta: os nós já fatiados (`bindings`, `structural`, `text`, e os dois
outros) são baratos ou caros de cobrir por MC/DC real? E, separadamente: onde vivem as
mensagens de erro hoje — centralizadas ou espalhadas?
**Ferramenta**: `cargo llvm-cov` com MC/DC via toolchain nightly
(`RUSTFLAGS="-Cinstrument-coverage -Ccoverage-options=mcdc"`), `llvm-cov show
--show-mcdc`. Alternativa se a integração `cargo-llvm-cov` não aceitar directamente as
flags de MC/DC: invocar `rustc`/`llvm-profdata`/`llvm-cov` directamente, documentando os
comandos exactos usados.
**Pré-condição**: `git status` limpo. Confirmar toolchain nightly disponível
(`rustup toolchain list`); instalar só se necessário, documentar a versão usada.

---

## Aviso a aplicar a todo o resultado, não só reportar no fim

**Bug conhecido do LLVM em Rust** (issue `llvm/llvm-project#109944`, Setembro 2026):
*constant folding* em expressões booleanas produz MC/DC **artificialmente alto** — uma
decisão que devia medir 33% pode sair a 50%. **Para qualquer decisão onde o número pareça
bom demais** (100% com poucos testes, por exemplo), verificar manualmente se alguma
condição envolvida é simplificável em tempo de compilação (literal booleano, constante,
`cfg!()`) antes de aceitar o número. Não reportar cobertura MC/DC sem esta verificação nos
casos suspeitos.

---

## Fase A — Alvo 1: os 5 nós já fatiados

Escolher, dentro de cada família já fatiada, **as funções que mais decisões combinadas
têm** (não a amostra aleatória — usar `grep`/inspecção rápida para achar `if`/`match` com
`&&`/`||` múltiplos antes de escolher, para o teste não sair artificialmente barato por
acaso):

1. `compiler/eval/bindings/` (P1013)
2. `compiler/stdlib/structural/` (P1014/P1023)
3. `compiler/stdlib/text/` (P1022)
4. `compiler/eval/operators/` (P1002) — incluir mesmo sendo o caso "puro", para
   comparação directa barato-vs-caro dentro do mesmo conjunto.
5. `compiler/stdlib/foundations/` (P1032)

Para cada família: instrumentar, correr a suite de testes existente (não escrever testes
novos ainda — o objectivo é medir o que **já** está coberto), extrair percentagem MC/DC
por função, aplicar o aviso do bug de constant folding a qualquer resultado suspeito.

## Fase B — Alvo 2: onde vivem as mensagens de erro

Separado da Fase A, mais simples:

```
grep -rn 'SourceDiagnostic::error\|fn.*error.*Diagnostic\|struct SourceDiagnostic' 01_core/src entities
```

Confirmar: existe um ponto central (tipo/função única que toda mensagem de erro passa),
ou cada `native_*`/ponto de erro constrói a sua própria mensagem inline, à semelhança do
padrão que o vanilla usa? Se houver ponto central, medir também o MC/DC dele nesta
mesma fase — é candidato natural a decisão complexa (framing de erro, hints, per-idioma
no futuro).

## Output

Tabela por família/função: nº de decisões, nº de condições combinadas na decisão mais
complexa, % MC/DC medido, se o resultado foi sinalizado como suspeito (bug de constant
folding) e a verificação manual feita nesse caso. Secção separada com a resposta da Fase
B (centralizado ou disperso, com file:line).

## O que este passo NÃO faz

- Não escreve testes novos para preencher lacunas encontradas.
- Não decide nada sobre i18n de erros — só confirma onde as mensagens vivem hoje.
- Não corrige nenhum código.

---

## Resultado esperado

Resposta com número, não impressão, à pergunta "os 5 nós são baratos ou caros de cobrir
por MC/DC" — família a família, não uma média única que esconda a variação. E resposta
factual a "as mensagens de erro estão centralizadas?", para dimensionar o ensaio de i18n
antes de o desenhar.
