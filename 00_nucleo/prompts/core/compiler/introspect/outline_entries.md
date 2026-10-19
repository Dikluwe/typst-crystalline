# L0 — introspect/outline_entries
Hash do Código: 9bb45cb9

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/outline_entries.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:922-988` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Materializar entradas elegíveis depois da travessia, mantendo a Location causal do heading e a Location própria da entry, os limites de profundidade, o prefixo e o fill registrados.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/core/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.


## Entradas diretas e ordem da síntese — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; order-before.json.gz/before.json.gz preservam estado, UTC, hashes e diffstat. trace-runtime-result.json e trace-runtime.log mostram a entry direta no índice puro, runtime e L3 sob NativeElement; a linguagem bare converte outline.entry em Kind(OutlineEntry), conforme foundations/selector.rs:113. O walk terminal atual não preenche esse kind. measurement-order-before.json no pin 9f2b6e8715237cb086899a42873660fe744622e8 mede BARE [Heading] versus [Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2 versus 1 e COUNTER ((2,),) versus ((1,),) na Location da entrada sintetizada. Esses são observáveis de linguagem; não se exige igualdade Rust/bytes. A síntese pós-walk aloca entradas depois dos updates, mas sua posição causal é dentro do outline anterior. O núcleo realization-order já proíbe tratar identidade numérica como ordem. A correção é refutada por ordem, prefixo temporal, duplicação ou dados de entry errados.

### Responsabilidade exclusiva

Conservar a síntese na fase pós-walk, os mesmos campos, filtros, depth, prefix, fill, Location própria e Location causal do heading. Usar a ordem dos outlines em outline_elements alinhada a kind_index Outline para identificar a âncora real por índice, sem igualdade Rust/nomes/hashes de conteúdo e sem truncar a síntese. Colher as Locations recém-geradas por outline e registrar sua ordem imediatamente após a Location do outline, na ordem dos headings, por helpers internos de CounterRegistry e StateRegistry. Preservar todas as Locations e a ordem relativa das tags existentes; apenas a posição causal das ocorrências sintetizadas é integrada. Se faltar âncora em snapshot legado parcial, conservar o fallback anterior, sem omitir entradas. Ao fim, ordenar o kind_index OutlineEntry pela ordem causal registrada, tornando bare/where/nativo consistentes, inclusive múltiplos outlines e uma entry direta entre eles. Não reconstruir ou reexecutar o documento, não gerar nós extras, mudar fase, cardinalidade ou fórmula de campos.

### Aceitação

Testes funcionais antes da mudança: índice bare da entry própria e aliases; ordem de três entries (dois outlines com entry direta entre eles), where e ausência de heading fantasma; estado e contador lidos na Location sintetizada devem refletir o prefixo do outline. Depois, testes de contrato dos dois helpers internos antes da sua implementação, com âncora desconhecida, duplicatas/repetição, IDs não monotônicos e append denso. Repetir nove sondas CLI sem relaxar resultados (seis originais e order/state/counter). Suíte completa, build, fmt, Clippy estrito, Rustdoc estrito, V1/V5/V15/V26/reversa/diff e SARIF: seis V16 removidos, sete V19 previstos e três mensagens existentes atualizadas, nenhum V20 novo. 12 GiB, swap zero, um job e comandos pesados sequenciais. Esta é correção interna/paridade ADR-0127, com helpers crate-private e nove pares L0/consumer; nenhuma assinatura externa, default deliberado ou fase nova. Resselo restrito, sem commit/push/release. As demais pendências de V16 continuam abertas.
