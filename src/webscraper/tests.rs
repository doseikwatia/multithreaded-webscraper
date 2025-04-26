use super::*;

#[tokio::test]
async fn get_google_test() {
    let url = "https://www.ghanaweb.com/GhanaHomePage/NewsArchive/No-Fufu-In-Kumasi-Sieme-Preko-tops-all-282";
    match get_webpage(url).await {
        Ok(content) => {
            println!("{}",content);
            assert_ne!(content.len(),0);
        },
        Err(err) => println!("Something weent wrong {}", err),
    }
}
#[tokio::test]
async fn parse_article_test(){
    let url = "https://www.ghanaweb.com/GhanaHomePage/NewsArchive/Volta-Regional-Police-seize-ammunition-on-Accra-Benin-bound-bus-1980671";
    let _ = get_webpage(url).await.and_then(|html|{
        Ok(parse_article(&html,url).unwrap())
    }).and_then(|article|{
        println!("article {:?}",article);
        Ok(())
    });
}

#[tokio::test]
async fn get_list_of_articles_test(){
    let html = r#"

</div>

<div class="bottom_artl_wrap">
  <div class="left_artl_list more_news">
    <div class="upper">      
      <div id="inner-left-col">
            <span class="page_title">General News</span>
          </div><ul><li>
            <a title="HOUSE DISCUSSES DRUG PROBLEM 
" href="/GhanaHomePage/NewsArchive/HOUSE-DISCUSSES-DRUG-PROBLEM-348">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>HOUSE DISCUSSES DRUG PROBLEM 
</p>
              </div>
            </a>
        </li><li>
            <a title="Ghana May host ECOWAS meeting after all 
" href="/GhanaHomePage/NewsArchive/Ghana-May-host-ECOWAS-meeting-after-all-347">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Ghana May host ECOWAS meeting after all 
</p>
              </div>
            </a>
        </li><li>
            <a title="MORE WOMEN URGED TO DO ENGINEERING 
" href="/GhanaHomePage/NewsArchive/MORE-WOMEN-URGED-TO-DO-ENGINEERING-346">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>MORE WOMEN URGED TO DO ENGINEERING 
</p>
              </div>
            </a>
        </li><li>
            <a title="Back to basics policy has been successful, claims Minister 
" href="/GhanaHomePage/NewsArchive/Back-to-basics-policy-has-been-successful-claims-Minister-345">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Back to basics policy has been successful, claims Minister 
</p>
              </div>
            </a>
        </li><li>
            <a title="NDC calls for mobilisation to register for 1996 
" href="/GhanaHomePage/NewsArchive/NDC-calls-for-mobilisation-to-register-for-1996-344">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>NDC calls for mobilisation to register for 1996 
</p>
              </div>
            </a>
        </li><li>
            <a title="Speaker calls for debate on taxation 
" href="/GhanaHomePage/NewsArchive/Speaker-calls-for-debate-on-taxation-343">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Speaker calls for debate on taxation 
</p>
              </div>
            </a>
        </li><li>
            <a title="USE SCIENCE TO PROMOTE SOCIO-ECONOMIC GROWTH 
" href="/GhanaHomePage/NewsArchive/USE-SCIENCE-TO-PROMOTE-SOCIO-ECONOMIC-GROWTH-342">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>USE SCIENCE TO PROMOTE SOCIO-ECONOMIC GROWTH 
</p>
              </div>
            </a>
        </li><li>
            <a title="Moderate wage demands 
" href="/GhanaHomePage/NewsArchive/Moderate-wage-demands-341">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Moderate wage demands 
</p>
              </div>
            </a>
        </li><li>
            <a title="BUSUNYA STUDENTS FARM TO GENERATE FUNDS FOR SCH 
" href="/GhanaHomePage/NewsArchive/BUSUNYA-STUDENTS-FARM-TO-GENERATE-FUNDS-FOR-SCH-340">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>BUSUNYA STUDENTS FARM TO GENERATE FUNDS FOR SCH 
</p>
              </div>
            </a>
        </li><li>
            <a title="Tuberculosis treatments centres for Brong Ahafo 
" href="/GhanaHomePage/NewsArchive/Tuberculosis-treatments-centres-for-Brong-Ahafo-339">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Tuberculosis treatments centres for Brong Ahafo 
</p>
              </div>
            </a>
        </li><li>
            <a title="Research in sugar substitutes 
" href="/GhanaHomePage/NewsArchive/Research-in-sugar-substitutes-338">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Research in sugar substitutes 
</p>
              </div>
            </a>
        </li><li>
            <a title="The State of  the nation in 24 hours" href="/GhanaHomePage/NewsArchive/The-State-of-the-nation-in-24-hours-335">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>The State of  the nation in 24 hours</p>
              </div>
            </a>
        </li><li>
            <a title="Malaria Control Programme Launched 
" href="/GhanaHomePage/NewsArchive/Malaria-Control-Programme-Launched-334">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Malaria Control Programme Launched 
</p>
              </div>
            </a>
        </li><li>
            <a title="MIND THE KIDS! 
" href="/GhanaHomePage/NewsArchive/MIND-THE-KIDS-333">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>MIND THE KIDS! 
</p>
              </div>
            </a>
        </li><li>
            <a title="Demonstrations in Sixth form College 
" href="/GhanaHomePage/NewsArchive/Demonstrations-in-Sixth-form-College-332">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Demonstrations in Sixth form College 
</p>
              </div>
            </a>
        </li><li>
            <a title="Donation to varsities 
" href="/GhanaHomePage/NewsArchive/Donation-to-varsities-331">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Donation to varsities 
</p>
              </div>
            </a>
        </li><li>
            <a title="Does the Govt plan to attack Free Press? 
" href="/GhanaHomePage/NewsArchive/Does-the-Govt-plan-to-attack-Free-Press-328">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Does the Govt plan to attack Free Press? 
</p>
              </div>
            </a>
        </li></ul><div id="inner-left-col">
            <span class="page_title">Crime & Punishment</span>
          </div><ul><li>
            <a title="Pastor jailed for abusing 10 year old girl" href="/GhanaHomePage/crime/Pastor-jailed-for-abusing-10-year-old-girl-330">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Pastor jailed for abusing 10 year old girl</p>
              </div>
            </a>
        </li><li>
            <a title="Raped and raped again?" href="/GhanaHomePage/crime/Raped-and-raped-again-329">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Raped and raped again?</p>
              </div>
            </a>
        </li></ul><div id="inner-left-col">
            <span class="page_title">Regional News</span>
          </div><ul><li>
            <a title="New Catholic Bishop for Mampong" href="/GhanaHomePage/regional/New-Catholic-Bishop-for-Mampong-337">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>New Catholic Bishop for Mampong</p>
              </div>
            </a>
        </li></ul><div id="inner-left-col">
            <span class="page_title">Editorial News</span>
          </div><ul><li>
            <a title="Social Security Bank" href="/GhanaHomePage/NewsArchive/Social-Security-Bank-317">
              <div class="image" style="background-color:#ececec;">
                
              </div>
              <div class="info">
                <p>Social Security Bank</p>
              </div>
            </a>
        </li></ul>
    </div>
  </div>
"#;
    let list = get_list_of_articles_from_page(html);
    assert_ne!(list.len(),0);
}