import fetch from 'node-fetch'
import fs from 'fs'

function compileDecentralizedSource(helperWebsites = null) {
  helperWebsites = []

  helperWebsites.push(fs.readFileSync('website_store/reddit.com.html', 'utf8').toString())
  helperWebsites.push(fs.readFileSync('website_store/amazon.com.html', 'utf8').toString())
  helperWebsites.push(fs.readFileSync('website_store/nytimes.com.html', 'utf8').toString())
  helperWebsites.push(fs.readFileSync('website_store/youtube.com.html', 'utf8').toString())
  helperWebsites.push(fs.readFileSync('website_store/www2.uottawa.ca.html', 'utf8').toString())
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
        console.log(`${sequenceIndex} ${commentForCompiling.length}`)
      }
    }
  }

  newCompiledWebsiteString = Buffer.from(newCompiledWebsiteString, 'utf8').toString()
  newCompiledWebsiteString.concat("\n<!-- Compiled using https://github.com/devanandersen/Censorship-Thesis -->")
  fs.writeFile("website_store/recompiled_website.html", newCompiledWebsiteString, function(err, result) {
    if (err) console.log('error', err)
  })
}

// Pulled from https://stackoverflow.com/questions/42108782/firefox-webextensions-get-local-files-content-by-path/44516256#44516256
function readFile(_path, _cb){
    fetch(_path).then(function(_res) {
        return _res.blob();
    }).then(function(_blob) {
        var reader = new FileReader();

        reader.addEventListener("loadend", function() {
            _cb(this.result);
        });

        reader.readAsText(_blob); 
    });
};


compileDecentralizedSource()
