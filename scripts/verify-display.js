#!/usr/bin/env node
// TEAM_320: GPU Display Verification Script
// Uses Puppeteer to connect to noVNC and check if display is black or has content.
//
// Usage: node scripts/verify-display.js [--timeout=30]
//
// Exit codes:
//   0 = Display has content (GPU working)
//   1 = Display is black (GPU broken)
//   2 = Error (connection failed, etc.)

const puppeteer = require('puppeteer');

const VNC_URL = 'http://localhost:6080/vnc.html';
const CONNECT_TIMEOUT = parseInt(process.argv.find(a => a.startsWith('--timeout='))?.split('=')[1] || '30') * 1000;

async function analyzeScreenshot(page) {
    // Get pixel data from the canvas
    const pixelData = await page.evaluate(() => {
        const canvas = document.querySelector('#noVNC_canvas');
        if (!canvas) return null;
        
        const ctx = canvas.getContext('2d');
        const width = canvas.width;
        const height = canvas.height;
        
        if (width === 0 || height === 0) return { error: 'Canvas has zero dimensions' };
        
        // Sample pixels across the canvas (every 50th pixel for performance)
        const imageData = ctx.getImageData(0, 0, width, height);
        const data = imageData.data;
        
        let nonBlackPixels = 0;
        let totalSampled = 0;
        
        // Sample grid of pixels
        for (let y = 0; y < height; y += 20) {
            for (let x = 0; x < width; x += 20) {
                const idx = (y * width + x) * 4;
                const r = data[idx];
                const g = data[idx + 1];
                const b = data[idx + 2];
                
                totalSampled++;
                // Check if pixel is not black (allow small tolerance for compression artifacts)
                if (r > 10 || g > 10 || b > 10) {
                    nonBlackPixels++;
                }
            }
        }
        
        return {
            width,
            height,
            totalSampled,
            nonBlackPixels,
            percentage: ((nonBlackPixels / totalSampled) * 100).toFixed(2)
        };
    });
    
    return pixelData;
}

async function main() {
    console.log('╔══════════════════════════════════════════════════════════╗');
    console.log('║  [GPU VERIFY] Starting display verification...           ║');
    console.log('╚══════════════════════════════════════════════════════════╝');
    
    let browser;
    try {
        browser = await puppeteer.launch({
            headless: true,
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        
        const page = await browser.newPage();
        await page.setViewport({ width: 1280, height: 800 });
        
        console.log(`[GPU VERIFY] Navigating to ${VNC_URL}...`);
        await page.goto(VNC_URL, { waitUntil: 'networkidle2', timeout: 10000 });
        
        // Click connect button
        console.log('[GPU VERIFY] Clicking Connect button...');
        await page.click('#noVNC_connect_button');
        
        // Wait for connection and display to stabilize
        console.log(`[GPU VERIFY] Waiting for display (${CONNECT_TIMEOUT/1000}s timeout)...`);
        await page.waitForSelector('#noVNC_canvas', { timeout: CONNECT_TIMEOUT });
        
        // Give the display a moment to render
        await new Promise(r => setTimeout(r, 3000));
        
        // Take screenshot for debugging
        await page.screenshot({ path: 'gpu_verify_screenshot.png', fullPage: true });
        console.log('[GPU VERIFY] Screenshot saved to gpu_verify_screenshot.png');
        
        // Analyze the display
        const result = await analyzeScreenshot(page);
        
        if (!result || result.error) {
            console.log('╔══════════════════════════════════════════════════════════╗');
            console.log('║  [GPU VERIFY] ERROR: Could not analyze display           ║');
            console.log(`║  ${result?.error || 'Canvas not found'}                  `);
            console.log('╚══════════════════════════════════════════════════════════╝');
            process.exit(2);
        }
        
        console.log('');
        console.log('╔══════════════════════════════════════════════════════════╗');
        if (result.nonBlackPixels > 10) {
            console.log('║  [GPU VERIFY] RESULT: DISPLAY HAS CONTENT ✅             ║');
            console.log(`║  Resolution: ${result.width}x${result.height}                          `);
            console.log(`║  Non-black pixels: ${result.nonBlackPixels}/${result.totalSampled} (${result.percentage}%)        `);
            console.log('╚══════════════════════════════════════════════════════════╝');
            process.exit(0);
        } else {
            console.log('║  [GPU VERIFY] RESULT: BLACK SCREEN ❌                     ║');
            console.log(`║  Resolution: ${result.width}x${result.height}                          `);
            console.log(`║  Non-black pixels: ${result.nonBlackPixels}/${result.totalSampled} (${result.percentage}%)        `);
            console.log('║  GPU is NOT rendering to display!                        ║');
            console.log('╚══════════════════════════════════════════════════════════╝');
            process.exit(1);
        }
        
    } catch (error) {
        console.log('╔══════════════════════════════════════════════════════════╗');
        console.log('║  [GPU VERIFY] ERROR: Verification failed                 ║');
        console.log(`║  ${error.message.substring(0, 50)}...                     `);
        console.log('╚══════════════════════════════════════════════════════════╝');
        process.exit(2);
    } finally {
        if (browser) await browser.close();
    }
}

main();
