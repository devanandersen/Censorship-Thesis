chrome.storage.local.get(['compiled_website'], function(result) {
  document.write(result['compiled_website'])
})
