# Passo 1372 — observador vertical discriminatório S07

## Objetivo

Substituir o comparador S07 confundido por textos diferentes por uma sonda com
carga textual equivalente e marcador pós-bloco mensurável.

## Sonda congelada

Ambos os casos usam página `200pt × 240pt`, margem `10pt`, um bloco de altura
`100%`, texto visual normalizado `Probe` e marcador posterior `After`.

```typst
// lone
#block(height: 100%)[#metadata("s07")<s07>Probe]
#parbreak()
After

// multiple-child control
#block(height: 100%)[P#box(width: 0pt)[]robe]
#parbreak()
After
```

O observer `pdftotext -bbox-layout` exige payload concatenado `ProbeAfter` e
exatamente um marcador `After`. A assinatura vertical inclui a posição
absoluta (`page`, `yMin`) do marcador em cada caso e a diferença entre lone e
controle, arredondada a 0,001 pt. A posição absoluta impede falso `MATCH`
quando ambos os casos de um produto ignoram igualmente a altura. Não usa hash
do texto como testemunha de expansão.

## Matriz e decisão

Executar dois casos × dois produtos × dois perfis (`a11y-extras`,
`html+a11y`): oito compilações e oito observações. Para cada perfil, o vanila
define a assinatura; o cristalino é `MATCH` somente se produzir a mesma
assinatura com observações completas. Ausência/duplicação do marcador, XML inválido,
texto diferente, timeout ou limite produz `Unknown`.

## Segurança

Reusar a fronteira limitada P1369: processos estritamente sequenciais,
`RLIMIT_AS=2 GiB`, `RLIMIT_FSIZE=16 MiB`, `RLIMIT_CPU=45 s`, timeout 45 s,
arquivos temporários e leitura máxima de 1 MiB por canal. Sem Cargo, build,
watch, rede deliberada ou suíte ampla. Uma execução preliminar que comparava
somente o delta foi rejeitada por falso `MATCH`; a revisão focal reutiliza a
evidência bruta e, se executar a matriz, conserva os mesmos limites.

## L0 e regime

Somente diagnóstico/harness muda. Protocolo completo executado sem atestação
de isolamento. O passo mede apenas expansão vertical S07; morfologia de tags é
um eixo separado e não é reclassificada aqui.
