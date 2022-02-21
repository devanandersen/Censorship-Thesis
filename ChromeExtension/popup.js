async function compileDecentralizedSource(helperWebsites = null) {
  helperWebsites = []
  urls = ['website_store/reddit.com.html', 'website_store/amazon.com.html', 'website_store/nytimes.com.html', 'website_store/www2.uottawa.ca.html', 'website_store/youtube.com.html']

  helperWebsites = await getHelperWebsites(urls)

  //let test = readFile('website_store/reddit.com.html', function(_res){
  //  helperWebsites.push(_res)
  //});
  //let test2 = readFile('website_store/amazon.com.html', function(_res){
  //  helperWebsites.push(_res)
  //});
  //let test3 = readFile('website_store/nytimes.com.html', function(_res){
  //  helperWebsites.push(_res)
  //});
  //let test4 = readFile('website_store/www2.uottawa.ca.html', function(_res){
  //  helperWebsites.push(_res)
  //});
  //let test5 = readFile('website_store/youtube.com.html', function(_res){
  //  helperWebsites.push(_res)
  //});

  //await Promise.all([test, test2, test3, test4, test5])

  for (let i = 0; i < helperWebsites.length; i++) { console.log(helperWebsites[i].length) }

  let newCompiledWebsiteString = []

  for (let helperWebsite of helperWebsites) {
    let commentRegex = new RegExp("<!--(.*?)-->")
    let commentList = helperWebsite.match(commentRegex)[1]
    let commentForCompiling = commentList.split(",")
    let helperWebsiteBytes = [...Buffer.from(helperWebsite, 'utf8')]
    if (commentForCompiling != [""]) {
      for (let sequenceIndex in commentForCompiling) {
        let newHelper = helperWebsite
        let sequenceMappings = commentForCompiling[sequenceIndex].split(":")
        let sequenceOriginLocation = sequenceMappings[0].split("-")
        let beginningChar = parseInt(sequenceOriginLocation[0])
        let endingChar = parseInt(sequenceOriginLocation[1])
        let length = endingChar - beginningChar
        let placementLocation = parseInt(sequenceMappings[1])

        if (newCompiledWebsiteString.length <= placementLocation+length) {
          let pushLength = (placementLocation + length) - newCompiledWebsiteString.length
          for (let i=0; i < pushLength-1; i++) {
            newCompiledWebsiteString.push(32)
          }
        }

        let sequenceToPrint = helperWebsiteBytes.slice(beginningChar, endingChar)
        if (newCompiledWebsiteString.length == placementLocation) {
          newCompiledWebsiteString = newCompiledWebsiteString.concat(sequenceToPrint)
        } else {
          for (let i = 0; i < length; i++) {
            newCompiledWebsiteString.splice(placementLocation+i, 1, sequenceToPrint[i])
          }
        }
        //console.log(`${sequenceIndex} ${commentForCompiling.length}`)
      }
    }
  }

  newCompiledWebsiteString = Buffer.from(newCompiledWebsiteString, 'utf8').toString()
  newCompiledWebsiteString.concat("\n<!-- Compiled using https://github.com/devanandersen/Censorship-Thesis -->")
  chrome.storage.local.set({"compiled_website": newCompiledWebsiteString})

  return newCompiledWebsiteString
}

async function getHelperWebsites(urls) {
  let helperWebsites = []
  let promisesArray = []

  for (let i=0; i<urls.length; i++) {
    promisesArray.push(readFile(urls[i], function(_res){
      helperWebsites.push(_res)
    }));
  }

  return new Promise((resolve) => { 
    Promise.all(promisesArray).then(() => {
      resolve(helperWebsites)
    })
  })
}

// Pulled from https://stackoverflow.com/questions/42108782/firefox-webextensions-get-local-files-content-by-path/44516256#44516256
async function readFile(_path, _cb){
    return new Promise((resolve) => { 
      fetch(_path).then(function(_res) {
          return _res.blob();
      }).then(function(_blob) {
          var reader = new FileReader();

          reader.addEventListener("loadend", function() {
              resolve(_cb(this.result));
          });

          reader.readAsText(_blob, 'utf8');
    })});
};

document.addEventListener('DOMContentLoaded', () => {
  let searchForm = document.getElementById("circumvented-search");

  searchForm.addEventListener("submit", async (e) => {
    e.preventDefault()
    // TODO: Add in code here to check for input box, and compile based on text provided
    await Promise.all([compileDecentralizedSource()])
    chrome.tabs.create({ url: chrome.runtime.getURL('recompiled_website.html') });
  });
})
