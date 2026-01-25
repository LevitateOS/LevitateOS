# TEAM_025: Hero Dark Theme Image Fix

## Status: Complete

## Objective
Fix the Hero component to display the correct themed image on initial load when the system is in dark mode.

## Problem
The Hero was using `useTernaryDarkMode` hook, but on initial render (before hydration), the hook doesn't have access to the system preference yet - it returns the default light mode value.

## Solution
Use a `mounted` state to detect when client-side JS has hydrated:
- **Before mount:** Use a `<picture>` element with `<source media="(prefers-color-scheme: dark)">` - this lets the browser natively select the right image based on system preference
- **After mount:** Use the hook value to handle explicit user preferences (from the theme toggle)

## Files Modified
- `website/src/components/Hero.tsx`

## Verification
- TypeScript passes
