# Passo 1281 — Recuperar a infraestrutura de prova de paridade

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Fazer a suíte completa voltar a compilar e tornar a matriz diferencial simétrica e
reproduzível antes de novas alegações de paridade.

## Baseline obrigatório

- Cristalino inicial auditado: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Vanilla ratificado: upstream/main `a51e02804`.
- Registar HEAD, `git status --short`, `git diff HEAD --stat`, hora e hashes dos
  binários usados em toda medição numérica.

## Escopo

1. Inventariar apenas os fixtures referenciados diretamente pelos testes e ausentes.
2. Decidir, com licença e proveniência verificadas, entre versionar, gerar ou substituir
   `lab/krilla-reference/assets/fonts/NotoSans-Regular.ttf`.
3. Restaurar ou gerar deterministicamente os casos `/.typ/sec_*.typ` exigidos por
   `03_infra/src/integration_tests.rs`.
4. Corrigir os padrões de `.gitignore` para que fixtures obrigatórios não possam sumir
   silenciosamente, sem reintroduzir artefatos de build ou caches.
5. Passar `--features html` simetricamente a vanilla e cristalino na matriz.
6. Atualizar expectativas obsoletas: um caso que agora coincide não permanece marcado
   como divergência esperada.

## Gates

- Auditar os Prompts L0 dos módulos de teste/harness afetados antes de alterar código.
- Se a solução mudar contrato público, comportamento padrão, fase de pipeline ou
  compatibilidade, atualizar L0 e parar para confirmação conforme ADR-0127.
- Não recuperar blobs indiscriminadamente do histórico; restaurar somente fixtures com
  função, origem e licença comprovadas.

## Teste RED → GREEN

1. Preservar a falha atual de compilação por fixture ausente como RED reproduzível.
2. Aplicar a menor correção que torne a suíte autocontida.
3. Demonstrar GREEN com:
   - `cargo test --workspace`;
   - testes unitários do runner;
   - matriz diferencial completa com flags simétricas;
   - `cargo build --workspace --bin typst`;
   - `crystalline-lint .`.

## Critério de fechamento

- Nenhum `include_bytes!`/`include_str!` produtivo ou de teste aponta para arquivo
  ausente ou inevitavelmente ignorado.
- A suíte compila e executa integralmente.
- O relatório separa falha de harness de diferença real da linguagem.
- A árvore termina sem artefatos não intencionais.

## Entrega ao Passo 1282

Registrar o comando canônico da matriz, hashes dos dois binários e a lista exata de
features aplicadas a cada lado.
