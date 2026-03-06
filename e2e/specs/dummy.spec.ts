describe('Test', () => {
   it('should pass', async () => {
      await browser.url('about:blank')
      expect(await browser.getTitle()).toBe('MacDiskTree')
   })
})
