import { readFile } from 'node:fs/promises';
import { expect, test } from '@playwright/test';

// Primeiro token do feed simulado (useSimulationFeed), exibido como "5K9a...3pLm".
const MINT = '5K9aXbU5vK14q8sW93eRtYu23pLm';
const SHORT_MINT = '5K9a...3pLm';
const PHASE_TIMEOUT = { timeout: 10_000 };

test('investidor conecta a carteira, acompanha um trade simulado e exporta o PnL', async ({ page }) => {
  const card = (title) => page.locator('div.rounded-xl', { hasText: title });
  const monitored = card('Oportunidades em Monitoramento');
  const positions = card('Posições Abertas em Execução');
  const history = card('Histórico de Trades Fechados');

  await test.step('US1 · Conectar carteira e ver endereço e saldo', async () => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Conectar Phantom' }).click();
    await expect(page.getByText('7xKX...gAsU')).toBeVisible();
    await expect(page.getByText('10.000 SOL')).toBeVisible();
  });

  await test.step('US3 · Iniciar o feed em tempo real', async () => {
    await page.getByRole('button', { name: /Iniciar Simulação/ }).click();
    await expect(page.getByRole('button', { name: /Pausar Demo/ })).toBeVisible();
  });

  const candidateRow = monitored.locator('tr', { hasText: SHORT_MINT });

  await test.step('US3 · Token descoberto entra em análise da IA', async () => {
    await expect(candidateRow.getByText('Analisando IA')).toBeVisible(PHASE_TIMEOUT);
  });

  await test.step('US3 · IA aprova e o bot abre posição de 0.1 SOL', async () => {
    await expect(candidateRow.getByText('Seguro (Gemini)')).toBeVisible(PHASE_TIMEOUT);
    await expect(positions.locator('tr', { hasText: SHORT_MINT })).toContainText('0.100 SOL');
  });

  await test.step('US3 · Take-profit (+50%) fecha a posição e entra no histórico', async () => {
    const closedRow = history.locator('tr', { hasText: SHORT_MINT });
    await expect(closedRow.getByText('Take-Profit')).toBeVisible(PHASE_TIMEOUT);
    await expect(closedRow).toContainText('+0.0500 SOL');
    await expect(positions.locator('tr', { hasText: SHORT_MINT })).toHaveCount(0);
    await page.getByRole('button', { name: /Pausar Demo/ }).click();
  });

  await test.step('US3 · Saída manual da posição restante', async () => {
    await positions.locator('tr', { hasText: 'SOL-CAT' }).getByRole('button', { name: 'Fechar' }).click();
    await expect(history.locator('tr', { hasText: 'SOL-CAT' }).getByText('Manual')).toBeVisible();
    await expect(positions.getByText('Nenhuma posição aberta no momento.', { exact: false })).toBeVisible();
    await expect(card('PnL Realizado')).toContainText('+0.1100 SOL');
  });

  await test.step('US8 · Exportar relatório de PnL em CSV', async () => {
    const [download] = await Promise.all([
      page.waitForEvent('download'),
      page.getByRole('button', { name: /Exportar Relatório CSV/ }).click(),
    ]);
    expect(download.suggestedFilename()).toMatch(/^trenchbot-pnl-\d+\.csv$/);
    const csv = await readFile(await download.path(), 'utf8');
    await test.info().attach('relatorio-pnl.csv', { body: csv, contentType: 'text/csv' });

    const lines = csv.trim().split('\n');
    expect(lines[0]).toBe('mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason');
    expect(lines).toHaveLength(5);
    expect(lines[1]).toMatch(/^9zP3kL14m8sW92eRtYu34qLm21p,\d+,\d+,100000000,130000000,30000000,0\.030000,30\.00,manual$/);
    expect(lines[2]).toMatch(new RegExp(`^${MINT},\\d+,\\d+,100000000,150000000,50000000,0\\.050000,50\\.00,take_profit$`));
  });
});
