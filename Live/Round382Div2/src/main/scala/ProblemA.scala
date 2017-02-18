import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemA {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val nkArray = line.split(" ").map(_.toInt)
    val n = nkArray(0)
    val k = nkArray(1)
    val layout = lines.next()
    val soln = solve(n, k, layout)
    bw.write(if (soln) "YES" else "NO")
    bw.newLine()
  }

  def solve(n: Int, k: Int, layout: String): Boolean = {
    val g = layout.indexOf('G')
    val t = layout.indexOf('T')
    if ((t - g) % k != 0) false
    else if (t > g) (g + k).until(t).by(k).forall(layout(_) == '.')
    else (g - k).until(t).by(-k).forall(layout(_) == '.')
  }
}
