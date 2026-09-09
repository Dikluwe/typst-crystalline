# Precisão da observação do freeze pelo implementador

O root leu o envelope provisório de `p1325-ab-freeze.json`, cujo campo `at`
era `01:01:56+00:00`, antes de aplicar C. A mensagem do autor com o hash final
`5efd24f3a37df5a42f6746e954922b04dacbec958b2877a025e812790bc5af01`
foi entregue durante/depois da chamada que aplicou C e iniciou GREEN.
A frase posterior do root dizendo ter lido esse hash final antes de C foi
imprecisa e é corrigida aqui, sem reescrever o histórico.

O root conferiu antes de C as identidades/hashes das entradas protegidas do
envelope: snippet R1, runner, comparador, expectativas integrais, baseline,
manifesto, núcleo e norma. Isso não significa leitura das asserções privadas:
a integração foi cega. Os hashes dessas entradas não mudaram; a diferença de envelope é só
o metadado temporal, documentada pelo autor em
`p1325-ab-freeze-metadata-note.md`. A revisão verificou a identidade das
entradas. Não atribuir ao implementador uma leitura antecipada do envelope
final que ele não fez. A/B segue sem atestação técnica de isolamento ou selo
de refinamento; esta nota não é um novo freeze de expectativas após C.
