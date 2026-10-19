# Prompt — typst-passo-847: eliminar arquivos de código com mais de um header `@prompt` (regra violada, não bug de ferramenta)

**Origem**: recorrência do mesmo sintoma em pelo menos quatro relatórios diferentes (P836, P837, P838, P845) — `crystalline-lint --fix-hashes` escreve o hash errado quando um arquivo `.rs` tem dois headers `@prompt`, cada vez corrigido manualmente. Confirmado pelo dono: **arquivos com dois `@prompt` nunca deveriam existir.** O problema não é a ferramenta reagir mal — é o estado do arquivo já estar fora da regra antes da ferramenta tocar nele.
**Estado**: aguardando execução — prioridade alta, porque cada passo novo que tocar um desses arquivos corre o risco de repetir a correção manual, ou pior, de não notar e deixar um hash errado sem detecção.

---

## Por que isto é diferente de um bug de lint comum

Um lint que corrige mal um caso raro é um problema de qualidade da ferramenta. Isto é outra coisa: o `crystalline-lint` nunca foi projetado para arquivos multi-`@prompt`, porque a regra do projeto não permite que eles existam — então quando um aparece (por engano de algum passo anterior), a ferramenta faz algo indefinido (escreve o hash certo no header errado) em vez de recusar. Ninguém decidiu formalmente permitir dois `@prompt` por arquivo; eles foram surgindo como efeito colateral de passos que estenderam um arquivo de código já existente sem perceber que ele estava ligado a mais de um L0.

## Passo 1 — Levantar todos os casos

1. Buscar em todo o código (`01_core/`, `03_infra/`, `02_shell/`, `04_wiring/`) por arquivos com mais de uma linha `@prompt` no header. Comando sugerido: `grep -rl '@prompt ' --include='*.rs' . | xargs -I{} sh -c 'echo -n "{}: "; grep -c "@prompt " {}' | awk -F': ' '$2 > 1'` (ajustar ao padrão real do header, conferir primeiro com `grep -A2 '@prompt' <um arquivo conhecido>`).
2. Para cada arquivo encontrado, listar: quais L0s ele referencia, e por que motivo histórico (qual passo introduziu o segundo `@prompt` e por quê — normalmente porque um passo adicionou funcionalidade de outro domínio dentro de um arquivo que já pertencia a um L0 diferente).
3. Confirmar a lista contra os quatro já conhecidos (`rules.rs`, `fallback_fonts.rs`, e os outros dois mencionados sem nome exato nos relatórios de P836/P837/P838) e ver se há mais não detectados por não terem gerado erro visível ainda.

## Passo 2 — Resolver cada caso (não patchear a ferramenta para aceitar dois)

Para cada arquivo com mais de um `@prompt`, duas soluções possíveis — decidir caso a caso, não aplicar a mesma tacada em todos:

1. **Dividir o arquivo de código** em dois arquivos, um por L0, se as responsabilidades forem separáveis com custo razoável (é o caminho mais alinhado com a regra: um arquivo, um prompt).
2. **Fundir os dois L0** em um só, se as duas responsabilidades hoje forem, na prática, uma coisa só que só cresceu por decisões incrementais (menos provável, mas registrar se for o caso — precisa de justificativa clara, não é a saída fácil por padrão).

Não escolher a solução mais barata sem justificar — registrar a razão da escolha para cada arquivo no relatório.

## Passo 3 — Corrigir os hashes definitivamente

Depois de cada arquivo ter voltado a ter só um `@prompt` (via divisão ou fusão), rodar `crystalline-lint --fix-hashes .` normalmente — sem correção manual, porque o caso que causava o comportamento indefinido não existe mais.

## Passo 4 — Endurecer o lint para não deixar isso acontecer de novo

Adicionar ao `crystalline-lint` uma verificação nova: se um arquivo `.rs` tiver mais de um header `@prompt`, isso é erro de lint (bloqueante, mesma categoria de V3/V4/V5), não silêncio nem correção automática ambígua. Isso transforma qualquer recorrência futura em um erro visível imediato, em vez de um hash sutilmente errado que só aparece se alguém notar.

## Passo 5 — Validação

1. `crystalline-lint .` limpo, sem os V7 de sempre e sem nenhum arquivo multi-`@prompt` restante.
2. `cargo test --workspace` completo — confirmar que dividir/fundir arquivos não quebrou nenhum teste (os testes devem simplesmente mudar de arquivo, não de comportamento).
3. Se o Passo 4 foi implementado, testar deliberadamente que o novo lint pega um arquivo multi-`@prompt` sintético (criar um caso de teste temporário, remover depois).

## Relatório

`00_nucleo/diagnosticos/typst-passo-847-relatorio.md` com: a lista completa de arquivos encontrados (Passo 1), a decisão e justificativa para cada um (Passo 2), confirmação de que os hashes agora fecham sem intervenção manual (Passo 3), e o que foi adicionado ao lint (Passo 4).
