# Trenchbot 🚀

## 👥 Equipe

* **[Daniel Canton Alvim Moreira]** – Full Stack
* **[Thiago Felipe Alves do Carmo]** – Full Stack
* **[Gabriel Alves da Silva]** – Full Stack

## 🎯 Objetivo do Sistema

O **Trenchbot** é um agente autônomo de inteligência artificial projetado para operar na "trincheira" do mercado de criptomoedas, focando especificamente na negociação ágil de *memecoins* na blockchain da Solana. Seu objetivo é monitorar pools de liquidez em tempo real, identificar padrões em lançamentos de tokens e executar operações de compra e venda em frações de segundo. Utilizando a IA para análise de sentimento e métricas *on-chain*, o bot busca maximizar o lucro do usuário enquanto tenta identificar e mitigar os riscos de golpes (*rug pulls*).

## 🛠️ Tecnologias Utilizadas

* **Backend (Blockchain & Lógica Core):** Rust (para interação de altíssima performance com os *smart contracts* da Solana) e Python (para processamento de dados de mercado e lógica de IA).
* **Frontend:** React.js com Tailwind CSS (para um painel de controle simples, rápido e responsivo).
* **Banco de Dados:** PostgreSQL (para armazenamento estruturado do histórico de transações, logs e configurações do usuário).
* **Agente de IA:** Google Gemini (Agente Integrado) para análise de anomalias em contratos, tomada de decisão em cenários de alta volatilidade e otimização de estratégias de trade. 

## 📝 Histórias de Usuário

1. **Como um investidor**, quero conectar minha carteira da Solana (ex: Phantom) ao painel do bot para que ele possa executar as ordens de trade com meus fundos de forma segura.
2. **Como um usuário**, quero definir um limite de *stop-loss* e *take-profit* global para que o bot feche operações automaticamente e proteja meu capital.
3. **Como um trader**, quero visualizar um dashboard em tempo real com as moedas que o bot está monitorando no momento para acompanhar as oportunidades de mercado identificadas pela IA.
4. **Como um usuário**, quero configurar compras automáticas (*sniping*) de tokens recém-lançados de carteiras com pelo menos um lançamento anterior cujo Market Cap ATH superou Y, descartando imediatamente o token e a carteira se o total de tokens lançados pela carteira superar X.
5. **Como um investidor**, quero que o agente de IA escaneie automaticamente o contrato inteligente da *memecoin* buscando funções maliciosas (como *mint infinito* ou taxas abusivas) para evitar a compra de *scams*.
6. **Como um trader**, quero receber alertas instantâneos no Telegram a cada operação de compra ou venda executada pelo bot para me manter informado sem precisar ficar olhando o painel.
7. **Como um usuário**, quero definir um valor fixo de Solana (SOL) a ser gasto por transação para manter um gerenciamento de risco rigoroso e consistente.
8. **Como um investidor**, quero poder exportar um relatório do meu histórico de Lucro e Perda (PNL) a qualquer momento para conferir meus rendimentos e facilitar minha declaração de impostos.

## Motor de trading e simulação

As regras das histórias 2, 4 e 7 estão disponíveis em `src/trading`, com configuração JSON, replay local e descoberta ao vivo pelo WebSocket do PumpPortal. **As ordens são simuladas**: não conecta carteira nem envia swaps reais. Apenas lançamentos pump.fun são candidatos. O fluxo anterior de análise continua disponível ao executar sem argumentos.

```sh
cargo test
cargo run -- --paper examples/trading.json examples/events.jsonl
# Para consumir continuamente eventos JSONL pela entrada padrão:
cargo run -- --paper examples/trading.json -
# Histórico completo de deployments pump.fun (requer HELIUS_API_KEY):
cargo run -- --scan-creator examples/trading.json CARTEIRA_SOLANA
# Descoberta ao vivo e ordens simuladas (HELIUS_API_KEY + PUMPPORTAL_API_KEY):
cargo run -- --pump-paper examples/trading.json examples/histories.json
```

O exemplo compra dois tokens, fecha um por take-profit e outro por stop-loss. A saída contém um resultado JSON por evento. Erros de evento/ordem são reportados e o processamento continua, permitindo avaliar saídas posteriores. Configuração inválida interrompe a inicialização. O saldo inicial simulado é 10 SOL, alterável por `PAPER_BALANCE_LAMPORTS` (inteiro).

### Configuração

Edite `examples/trading.json` ou forneça outro arquivo:

| Campo | Regra |
| --- | --- |
| `stop_loss_percent` | Percentual de perda por posição, maior que 0 e até 100. |
| `take_profit_percent` | Percentual de ganho por posição, positivo. |
| `sol_per_trade` | Valor fixo por compra em SOL, como string decimal (`"0.1"`), com até 9 casas; convertido exatamente em lamports. |
| `sniping_enabled` | Habilita novas compras; desabilitá-lo mantém as saídas automáticas. |
| `max_creator_launches` | X: máximo de tokens lançados pela carteira **na pump.fun**, incluindo o candidato, independentemente do ATH. |
| `min_ath_market_cap_usd` | Y em USD: pelo menos um token anterior deve ter ATH estritamente maior que Y. |
| `max_launch_age_seconds` | Idade máxima inclusiva do lançamento em relação ao instante de observação. |

Se o total de lançamentos ultrapassar X, o motor descarta imediatamente o candidato e bloqueia novos candidatos da carteira durante a sessão. Mesmo um histórico parcial pode provar esse excesso. Para aprovar uma compra, o histórico deve ser completo e pertencer ao criador informado. Registros duplicados são rejeitados; o candidato pode aparecer no histórico e é contado uma única vez. O candidato e tokens lançados depois dele não servem como sucesso anterior. ATH ausente/inválido impede a compra; market cap atual não substitui ATH.

Stop-loss e take-profit são configurações globais **aplicadas individualmente a todas as posições**, em relação ao custo de entrada em SOL. Ao atingir ou ultrapassar um limite, o motor solicita a venda de toda a quantidade. A função `Engine::configure` valida alterações e as aplica também às posições existentes. Carteiras bloqueadas continuam bloqueadas na sessão após reconfiguração; o bloqueio não impede saídas de posições abertas.

Cada compra usa o mesmo valor nominal em lamports. Saldo insuficiente impede a operação. A simulação não modela taxas, slippage, liquidez nem arredondamento por decimais do token. A detecção de saída depende da chegada de eventos de preço; não garante execução no preço do limite.

### Integração de dados e execução

`Event::Launch` recebe `platform` (apenas `pump_fun` é aceito), mint, carteira que realizou o lançamento, timestamps Unix em segundos, preço em SOL por token e histórico de deployments/ATH. `Event::Price` recebe mint, timestamp e preço em SOL; cotações fora de ordem são ignoradas. Veja o contrato JSON em `examples/events.jsonl`. Os identificadores `DEMO-*` são fictícios.

O adaptador histórico verifica deployments pump.fun pelo RPC archival da Helius. ATH em USD continua sendo fornecido separadamente, pelo arquivo de históricos. Os campos atuais de metadata/Jupiter não fornecem esse histórico e não são usados para inferi-lo. Em replay histórico, o provedor deve fornecer somente dados conhecidos naquele momento.

`Broker` separa as regras da execução. Um adaptador real deverá assinar/enviar swaps, usar o valor exato solicitado e retornar apenas fills confirmados, com a quantidade efetivamente recebida. Erros devem significar que nenhuma operação ocorreu; resultados ambíguos precisam ser reconciliados pelo adaptador antes de retornar. Ordens com erro preservam o estado para nova tentativa. Tokens comprados não são recomprados na mesma sessão, mesmo após a venda.

Posições e tokens comprados ficam em memória nesta versão. O cache SQLite preserva deployments confirmados, progresso das consultas e consumo diário; carteiras acima de X são rejeitadas novamente usando esse cache após reiniciar. O saldo simulado começa uma sessão nova a cada execução. Persistência de posições, reconciliação com a carteira, fonte automática de ATH e execução de swaps ainda são necessárias para operação real.

### WebSocket pump.fun

`--pump-paper` usa uma única conexão com `wss://pumpportal.fun/api/data`, assina `subscribeNewToken` e aceita somente eventos `pool: "pump"`. `traderPublicKey` em um evento `create` inicia a consulta; a atribuição do deployment é verificada no RPC. Compradores posteriores não são tratados como criadores. O preço estimado usa a razão entre reservas virtuais SOL/token.

A consulta histórica roda em uma thread separada, com no máximo 16 candidatos pendentes, sem bloquear o processamento dos preços de posições abertas. Somente após verificar deployments e ATH, o bot assina os trades do candidato e espera uma cotação nova para simular a entrada. Candidatos expirados ou recusados têm a assinatura cancelada; posições abertas continuam monitoradas até a venda.

A conexão é refeita com espera progressiva até 30 segundos, repondo as assinaturas das posições abertas. Reconexões invalidam candidatos pendentes, inclusive respostas históricas que chegarem depois. Um novo deployment observado da mesma carteira também invalida candidatos anteriores. Novos candidatos passam novamente pela consulta incremental. Nenhuma chave aparece nos logs de conexão.

O arquivo de históricos é um objeto indexado pela carteira criadora:

```json
{
  "CARTEIRA_CRIADORA": {
    "creator": "CARTEIRA_CRIADORA",
    "complete": true,
    "tokens": [
      { "mint": "TOKEN_ANTERIOR", "launched_at": 1700000000, "ath_market_cap_usd": 2000000.0 }
    ]
  }
}
```

No modo ao vivo, esse arquivo fornece apenas a evidência de ATH, de uma fonte confiável. O campo `complete` e os timestamps do arquivo não comprovam deployments: contagem, criador e datas vêm do RPC. Cada token anterior enumerado precisa ter ATH disponível no arquivo; ausência ou valores inválidos bloqueiam a compra. Não se infere ATH usando o preço atual. O exemplo `histories.json` está vazio propositalmente e não permite compras.

O WebSocket fornece dados ao vivo, não o histórico de deployments/ATH ([FAQ do provedor](https://pumpportal.fun/FAQ/)). Além disso, assinaturas de trades exigem chave e têm cobrança do provedor, mesmo com ordens simuladas ([preços](https://pumpportal.fun/fees/)). A descoberta de novos tokens é gratuita.

O adaptador cobre a curva pump.fun; preços após migração para PumpSwap ainda não são tratados. O stream é `processed`; o scanner exige encontrar o deployment em histórico `finalized` antes de liberar o candidato. Sua data de lançamento vem do bloco; a observação de preço usa o instante de recebimento. Esses limites, posições apenas em memória e fills simulados impedem tratar esse modo como execução real de produção.

### Consulta histórica e controle de custo

Defina `HELIUS_API_KEY` no ambiente. O endpoint é fixo no archival mainnet da Helius; um RPC público com histórico podado não pode comprovar a contagem. O comando `--scan-creator CONFIG.json CARTEIRA` imprime JSON com `complete`, `over_limit` e os deployments encontrados. Ele não compra nem consulta ATH. Pode ser usado para preparar o cache antes de acompanhar uma carteira ao vivo.

| Variável | Padrão | Uso |
| --- | --- | --- |
| `ARCHIVE_DB` | `creator-history.sqlite` | Cache SQLite compartilhado pelo scanner e modo ao vivo. |
| `ARCHIVE_DAILY_REQUESTS` | `25000` | Máximo de tentativas RPC por dia UTC, persistido no mesmo banco. |
| `HELIUS_API_KEY` | Sem padrão | Chave de acesso ao histórico archival mainnet. |

O scanner pagina `getSignaturesForAddress` e busca as transações com `getTransaction`, ambos com compromisso `finalized`. Decodifica `create` e `create_v2`, inclusive chamadas internas (CPI), e atribui cada mint ao campo `user` da instrução de criação. Esse critério identifica quem realizou o deployment, não o beneficiário mutável das taxas, o pagador genérico da transação ou quem possui o token hoje. A contagem inclui tokens sem sucesso, migrados e não mais presentes nas listas atuais de ativos, mas exclui outras plataformas e transações que falharam.

- Ao confirmar mais de X mints distintos, encerra a consulta. O cache basta para rejeitar essa carteira nas próximas vezes, sem novas requisições.
- Para comprovar até X, precisa esgotar o histórico archival. Uma página curta não é tratada como fim: consulta a página seguinte até receber uma lista vazia.
- Após completar a primeira varredura, guarda a assinatura mais recente como âncora e examina somente transações novas. A âncora precisa ser encontrada; sua ausência não autoriza compras.
- Deployments e cursor são gravados atomicamente por transação. Consultas interrompidas retomam do cursor e, ao terminar, atualizam os eventos ocorridos durante a interrupção. Aumentar X permite retomar uma carteira anteriormente rejeitada pelo limite menor.
- Dados ausentes, CPI não disponível, instruções pump desconhecidas, erro RPC ou orçamento esgotado mantêm a verificação incompleta. Não há fallback para listas parciais de tokens.

Uma carteira com poucos deployments pode ter muitas transações. Nesse caso, a primeira varredura ainda pode ser longa. No modo ao vivo, o prazo do candidato limita o trabalho; se a consulta não terminar a tempo, o bot ignora a oportunidade e preserva o progresso. A fila cheia também ignora candidatos, sem aprová-los com informação parcial. A confirmação final adiciona latência ao sniping.

Há intervalo mínimo de 150 ms entre requisições do worker. O orçamento conta tentativas antes de enviá-las, inclusive erros, e continua valendo após reiniciar. O limite é local ao banco: outras aplicações, chaves compartilhadas e bancos diferentes não entram nessa contagem. Apagar o banco apaga também o histórico de consumo. Evite habilitar cobrança automática adicional no provedor se desejar permanecer apenas na franquia gratuita.

Em 29/09/2026, a Helius documenta 1 crédito por chamada archival desses dois métodos e 1 milhão de créditos mensais no plano gratuito. O padrão de 25 mil tentativas/dia consome no máximo 775 mil em 31 dias, reservando margem; não cobre gastos com PumpPortal nem outras aplicações. Consulte os [créditos da Helius](https://www.helius.dev/docs/billing/credits) para preços vigentes.

Referências: [histórico archival Helius](https://www.helius.dev/historical-data), [paginação Solana](https://solana.com/docs/rpc/http/getsignaturesforaddress), [getTransaction](https://solana.com/docs/rpc/http/gettransaction), [IDL oficial pump.fun](https://github.com/pump-fun/pump-public-docs/blob/main/idl/pump.json). O decoder está fixado nas instruções revisadas em 29/09/2026; atualizações de programa podem exigir revisão antes de continuar.
