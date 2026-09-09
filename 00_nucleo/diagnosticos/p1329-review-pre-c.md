# P1329 — freeze e integração antes de C

Revisor `/root/p1329_review`, A/B sem atestação técnica de isolamento/refinement
seal. **Freeze R2 e integração aprovados; C permanece condicionado ao RED.**

Auditoria independente em `2026-09-09T12:36:18.190Z`, sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Estado integral e diff/stat anterior/posterior constam no recibo de integração
SHA-256 `338d6db6ef72a5b09b1014c4a91d18f7c540c479271c8e8a483aaaa454b77e1c`.

Entradas:

- manifesto R2 `c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1`;
- freeze R2 `1f649122c8a95b2b153db8cb88bb2711188c7a853eed5381f0cc18bb4418eba0`;
- expected R2 `9501ae018ada17565bfc54227c244bea9af99e3988c9a212b262e92a0ee7c239`;
- baseline CLI r1 `b89049333f76a7f49b2828a6ced0fc0c7ba747f6b7ca9ea019fba5a27e0bd91d`.

O revisor leu o freeze R2 e verificou por comparações independentes:

- Todos os hashes do mapa `frozen` da integração correspondem aos arquivos.
- Expected R1 e R2 têm listas `expectations` idênticas integralmente.
- As chaves caso/perfil são únicas, sem perda ou duplicação, nos 252 observáveis
  baseline/expected; ambos os conjuntos de chaves são iguais.
- Os 112 observáveis P1328 preservam expressão e perfil. Das expectativas
  integrais exit/stdout/stderr, 96 igualam o candidato P1328; apenas 16, os
  quatro tipos dimensionais nos quatro perfis, passam ao vanilla pinado.
- O owner atual é exatamente `original_owner` do baseline com substituição
  do antigo snippet pelos dois snippets congelados, permitindo somente troca
  da linha de `@prompt-hash`. Nenhuma implementação funcional P1329 entrou.
- Inventários produtivos antes/depois da integração não mudam fora de owner/L0.
- O diff do runner R2 contém somente referências a expected/manifesto R2 e
  o hash da norma; catálogo e comparador permanecem iguais.

O freeze R2 reconhece explicitamente NaN dimensional como obrigação local e
retém a dívida pública de construção anterior. Os mesmos bytes de testes
continuam admissíveis nesse domínio esclarecido. Não houve reescrita de r1.

Pendente: RED deve mostrar falhas de asserção por suporte dimensional/erro
misto ausente, sem ser confundido com falha de compilação, ambiente ou fixture.
Depois de C, GREEN dos mesmos bytes e todas as células CLI devem passar;
o exit zero isolado do runner não é gate suficiente.
