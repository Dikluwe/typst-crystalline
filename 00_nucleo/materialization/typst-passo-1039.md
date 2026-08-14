# Passo 1039 — Tentativa `gnatcov` para MC/DC real + correção do relatório do Passo 1038

**Tipo**: Duas partes. Parte 1: corrigir o relatório do Passo 1038 (métrica mal
identificada, coluna sem sentido para o tipo de dado obtido). Parte 2: tentar `gnatcov`
como caminho alternativo para MC/DC real em Rust, via flag estável `-C` (não `-Z`
nightly, que já falhou no P1038).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1038.

---

## Parte 1 — Corrigir o relatório do Passo 1038

1. A tabela da Fase A mede **cobertura de linha/função/região**, não MC/DC. Renomear o
   título da secção e das colunas para reflectir isto com precisão (ex.: "Cobertura de
   Linha/Função/Região" em vez de qualquer menção a MC/DC na tabela em si).
2. A coluna "Suspeita de Constant Folding?" não se aplica a este tipo de métrica (o bug
   `llvm/llvm-project#109944` é especificamente sobre contagem de pares de independência
   de condições MC/DC, que não existe em cobertura de linha/região). Remover a coluna, ou
   substituir por nota explícita: "Não aplicável — esta medição não é MC/DC."
3. Manter a Fase B como está — essa parte foi aceite sem reserva.
4. Registar no topo do relatório corrigido: "Tentativa de MC/DC real via `-Z
   coverage-options=mcdc` falhou (nightly `1.99.0-nightly` só aceita `block | branch |
   condition`); os números abaixo são cobertura de linha/função/região, métrica mais
   fraca que MC/DC. Ver Passo 1039 para tentativa alternativa via `gnatcov`."

## Parte 2 — Tentar `gnatcov`

### Fase A — Confirmar disponibilidade

```bash
which gnatcov || echo "não instalado"
gnatcov --version 2>&1 || true
```

Se não estiver instalado: verificar se está disponível via package manager do sistema, ou
se requer download da AdaCore (GNAT Community/Pro). Documentar o caminho de instalação
tentado e o resultado — se a instalação for complexa ou exigir licença, **parar e
reportar**, não gastar tempo excessivo a contornar uma barreira de licenciamento.

### Fase B — Se disponível, tentar o caminho documentado

```bash
RUSTFLAGS="-Cinstrument-coverage -Ccoverage-options=mcdc" cargo build --workspace
# gerar .profraw correndo os testes
LLVM_PROFILE_FILE="trace-%p.profraw" cargo test --workspace
gnatcov coverage --level=stmt+mcdc -a xcov --exec <binário-alvo> *.profraw
```

Nota: a flag aqui é `-C` (estável), diferente da `-Z coverage-options=mcdc` que falhou no
P1038 (essa era nightly-only e rejeitada). Confirmar se `-C instrument-coverage` com
`-Ccoverage-options=mcdc` é aceite pelo compilador estável ou também exige nightly —
reportar o erro exacto se falhar, mesma disciplina do P1038.

### Fase C — Se funcionar, medir os mesmos 5 nós

Repetir a medição da Fase A do Passo 1038 (os 5 nós, funções com mais decisões
combinadas), desta vez com MC/DC real. Aplicar o aviso do bug de constant folding
(`llvm/llvm-project#109944`) a qualquer resultado suspeito — verificar manualmente os
casos onde o número sair mais alto do que os testes existentes fariam prever, mostrando o
raciocínio, não só a conclusão.

### Fase D — Se não funcionar

Documentar exactamente onde falhou (instalação, flag rejeitada, incompatibilidade de
versão do LLVM entre `gnatcov` e o toolchain Rust, ou outra). Não insistir além de uma
tentativa directa — se `gnatcov` também não estiver maduro para este caso, aceitar isso
como resposta ("MC/DC real não está disponível hoje para Rust neste ambiente, com as
ferramentas tentadas") e cair para a alternativa estática (contagem de condições por
decisão, fórmula N+1, sem instrumentação) como próximo passo, não tentar mais caminhos
sem confirmar primeiro.

---

## Resultado esperado

Relatório do P1038 corrigido, sem alegar MC/DC onde mediu outra coisa. Resposta definitiva
sobre se `gnatcov` é viável neste ambiente — MC/DC real medido nos 5 nós, ou confirmação
clara de que não é possível hoje, com a razão exacta documentada.
