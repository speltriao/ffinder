# ffinder
Fast Rust CLI program for smart-finding files on Linux.
<br></br>
<p><b>Basic usage:</b> <a href="https://github.com/speltriao/ffinder/releases/download/0.1/ffinder"> Download </a> ffinder, drag and drop it to a terminal and simply type the file that you are searching.</p>  
Ex: <blockquote>ffinder My_File</blockquote> 
<br>
<p>The search paramater doesn't need to be identical to the file being searched, ffinder tries to be flexible. It also ignores some special characters and intelligently transform others (such as '-' , '_'  and   ',').</p>


<br></br>
<p><b>Advanced usage:</b></p>
<ol>
  <li>The default base directory is your home. To specify another, simply type it.  Ex:<i> ffinder </i> <b>/usr/bin/my_dir</b> <i>My File</i>
  <li>To search inside the current directory (cd command), specify the <b>-c</b> flag. Ex:<i> ffinder </i> <b>-c</b> <i>My File</i></li>
  <li>To expand the search to Hidden Directories, specify the <b>-h</b> flag. Ex:<i> ffinder </i> <b>-h</b> <i>My File</i></li>
  <li>If the file extension is especified, ffinder will only return results with the given extension.</li>
</ol>
<br></br>
<p><b><i>Optional</b></i>: Copy ffinder to /usr/bin to call it anytime:</p>
<p><blockquote>git clone https://github.com/speltriao/ffinder && cd ffinder && chmod +x ffinder && sudo cp ffinder /usr/bin</blockquote></p>
<p> After, just open a terminal and use ffinder normally.</p>

