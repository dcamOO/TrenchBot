import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 90_000,
  reporter: [['list', { printSteps: true }], ['html', { open: 'never' }]],
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'retain-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    {
      // Apresentação: navegador visível, ações em câmera lenta, vídeo e trace sempre gravados.
      name: 'demo',
      use: {
        ...devices['Desktop Chrome'],
        headless: false,
        viewport: { width: 1440, height: 900 },
        launchOptions: { slowMo: 600 },
        video: { mode: 'on', size: { width: 1440, height: 900 } },
        trace: 'on',
      },
    },
  ],
  webServer: {
    command: 'npm run dev -- --strictPort',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
