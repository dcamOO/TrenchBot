import { readFile } from 'node:fs/promises';
import { expect, test } from '@playwright/test';

// Primeiro token do feed simulado (useSimulationFeed), exibido como "5K9a...3pLm".
const MINT = '5K9aXbU5vK14q8sW93eRtYu23pLm';
const SHORT_MINT = '5K9a...3pLm';
const PHASE_TIMEOUT = { timeout: 10_000 };

test('investidor conecta a carteira, acompanha um trade simulado e exporta o PnL', async ({ page }) => {
  await page.goto('/');

  const card = (title) => page.locator('div.rounded-xl', { hasText: title });
  const monitored = card('Oportunidades em Monitoramento');
  const positions = card('Posições Abertas em Execução');
  const history = card('Histórico de Trades Fechados');

  // US1: conectar carteira (modo paper) e ver endereço e saldo.
  await page.getByRole('button', { name: 'Conectar Phantom' }).click();
  await expect(page.getByText('7xKX...gAsU')).toBeVisible();
  await expect(page.getByText('10.000 SOL')).toBeVisible();

  // US3: dashboard em tempo real acompanha descoberta, aprovação, compra e take-profit.
  await page.getByRole('button', { name: /Iniciar Simulação/ }).click();
  await expect(page.getByRole('button', { name: /Pausar Demo/ })).toBeVisible();

  const candidateRow = monitored.locator('tr', { hasText: SHORT_MINT });
  await expect(candidateRow.getByText('Analisando IA')).toBeVisible(PHASE_TIMEOUT);
  await expect(candidateRow.getByText('Seguro (Gemini)')).toBeVisible(PHASE_TIMEOUT);
  await expect(positions.locator('tr', { hasText: SHORT_MINT })).toBeVisible();

  const closedRow = history.locator('tr', { hasText: SHORT_MINT });
  await expect(closedRow.getByText('Take-Profit')).toBeVisible(PHASE_TIMEOUT);
  await expect(closedRow).toContainText('+0.0500 SOL');
  await expect(positions.locator('tr', { hasText: SHORT_MINT })).toHaveCount(0);
  await page.getByRole('button', { name: /Pausar Demo/ }).click();

  // Saída manual da posição inicial aparece no histórico como "Manual".
  await positions.locator('tr', { hasText: 'SOL-CAT' }).getByRole('button', { name: 'Fechar' }).click();
  await expect(history.locator('tr', { hasText: 'SOL-CAT' }).getByText('Manual')).toBeVisible();
  await expect(page.getByText('Nenhuma posição aberta no momento.', { exact: false })).toBeVisible();

  // US8: exportar o relatório CSV e conferir o conteúdo baixado.
  const [download] = await Promise.all([
    page.waitForEvent('download'),
    page.getByRole('button', { name: /Exportar Relatório CSV/ }).click(),
  ]);
  expect(download.suggestedFilename()).toMatch(/^trenchbot-pnl-\d+\.csv$/);
  const lines = (await readFile(await download.path(), 'utf8')).trim().split('\n');
  expect(lines[0]).toBe('mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason');
  expect(lines).toHaveLength(5);
  expect(lines[1]).toMatch(/^9zP3kL14m8sW92eRtYu34qLm21p,\d+,\d+,100000000,130000000,30000000,0\.030000,30\.00,manual$/);
  expect(lines[2]).toMatch(new RegExp(`^${MINT},\\d+,\\d+,100000000,150000000,50000000,0\\.050000,50\\.00,take_profit$`));
});
