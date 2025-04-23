# Introduction
I took the [Advanced Rust Programming](https://www.coursera.org/learn/advanced-rust-programming/supplement/ML6Ty/project-multithreaded-web-scraper-in-rust) class on Coursera and was tasked to build a webscraper that has the requirements described below. 
I decided to build one to pull all articles from [ghanaweb](https://www.ghanaweb.com), a popular news site in Ghana. My goal was not to pull the data but rather explore the skills acquired to pull the data in the most efficient way. 

# Project : Multithreaded Web Scraper in Rust
## Multithreaded Web Scraper in Rust

A multithreaded web scraper uses multiple threads built with Rust for web scraping. It fetches web pages and extracts specific information such as titles, meta descriptions, headings, and links.

## Problem Statement:

Build a web scraper in Rust that uses multiple threads to fetch web pages at once and extract specific details from each page. The scraper should be able to extract titles, meta descriptions, headings, and links from HTML content.

## Requirements:

## Concurrency: 

Use Rust's multithreading to get many web pages at once. 

Set up a thread pool or something like it to handle lots of HTTP requests at the  same time. 

Web Page Fetching:

Get HTML from web pages using HTTP requests. 

Deal with HTTP errors and problems. 

HTML Parsing: 

Break down the HTML you got to find this stuff: The web page's title. 

Descriptions in the meta tags. Headings (h1, h2, h3, and so on). Links (href bits from tags). 

Output

Show or save what you found neatly (like JSON or CSV). 

Keep track of what's happening and any problems while scraping. 

## Error Handling: 

Have a solid plan to handle network issues, parsing problems, and other things that might go wrong. 

Make sure the scraper can deal with weird or broken HTML without crashing. 

## Modular Design:

Build the scraper so each part (like networking, HTML parsing, and running things at the same time) is separate. 

Use Rust's way of organizing code to make parts you can reuse and fix.

Additional Guidelines

Utilize Rust's libraries for HTTP requests (e.g., request) and HTML parsing (e.g., select, html5ever).

Optimize performance by managing concurrency and resource utilization effectively.

Ensure the scraper respects robots.txt and site-specific rules to avoid unnecessary load on servers.


# Implementation

## Architecture
The system consists of 2 threadpools and the main threads. The main thread initializes the various threatpools and then starts generating archive page links for the first threadpool. Theads communicate with each other using channels, which are also setup by the main thread. The first threadpool fetches links to all articles and then generates `Article` structs which consist of the `date`, `title` and `content` of the article. These structs are sent to the second threadpool to be written to a json file.


```
                                    ┌───────────────────────────┐                                                                       
                                    │                           │                                                                       
                                    │    ┌─────────────────┐    │                                                                       
                                    │    │                 │    │                                                                       
                                    │    │ Worker  Thread  │    │                                                                       
                                    │    │                 │    │                                                                       
                                    │    └─────────────────┘    │            ┌───────────────────────────┐                              
                                    │                           │            │                           │                              
                                    │                           │            │    ┌─────────────────┐    │                              
                                    │    ┌─────────────────┐    │            │    │                 │    │                              
                                    │    │                 │    │            │    │ Worker  Thread  │    │                              
   ┌─────────────────┐              │    │ Worker  Thread  │    │            │    │                 │    │               ┌─────────────┐
   │                 │  channel     │    │                 │    │  channel   │    └─────────────────┘    │               │             │
   │   Main Thread   ┼──────────────►    └─────────────────┘    ┼────────────►                           ┼──────────────►│    File     │
   │                 │  links       │                           │  Article   │                           │               │             │
   └─────────────────┘              │    ┌─────────────────┐    │            │    ┌─────────────────┐    │               └─────────────┘
                                    │    │                 │    │            │    │                 │    │                              
Generate link to archive page       │    │ Worker  Thread  │    │            │    │ Worker  Thread  │    │                              
using dates                         │    │                 │    │            │    │                 │    │                              
                                    │    └─────────────────┘    │            │    └─────────────────┘    │                              
                                    │                           │            │                           │                              
                                    │                           │            │                           │                              
                                    │    ┌─────────────────┐    │            └───────────────────────────┘                              
                                    │    │                 │    │                                                                       
                                    │    │ Worker  Thread  │    │              Write Article struct to                                  
                                    │    │                 │    │              json file                                                
                                    │    └─────────────────┘    │                                                                       
                                    │                           │                                                                       
                                    │                           │                                                                       
                                    └───────────────────────────┘                                                                       
                                                                                                                                        
                                    Pull links to articles and pull                                                                     
                                    contents, title, and date into                                                                      
                                    Article struct                                                                                      
```