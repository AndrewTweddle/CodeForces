import java.io._

import scala.io.Source

object ProblemC {
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
  case class Move(index: Int, direction: Char) {
    override def toString() = s"$index $direction"
  }

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val n = lines.next().toInt
    val a = lines.next().split(' ').map(_.toInt)
    val k = lines.next().toInt
    val b = lines.next().split(' ').map(_.toInt)
    val solution = solve(n, k, a, b)
    solution match {
      case Some(moves) => {
        bw.write("YES")
        moves.foreach { move =>
          bw.newLine()
          bw.write(move.toString)
        }
      }
      case None => bw.write("NO")
    }
    bw.newLine()
  }

  /** The approach is to folder over the weights in the a matrix,
    * until the current group of a values matches the next b value, and track the largest monster in the group.
    * If there are multiple largest monsters, then the first will eat all monsters to its left then all to its right.
    * The exception is if the first of these largest monsters starts the group and has other largest monsters after it.
    * In this case, take the last largest monster in this initial group of largest monsters.
    * It will eat all monsters to its right, then all monsters to its left.
    * But beware... if all monsters in the group are identical, then there is no solution, as none can eat any other.
    *
    * The Accumulator class tracks the state after all previous weights (entries in a) have been evaluated.
    * In particular it tracks the current group (corresponding to the next b value to be matched).
    */
  case class Accumulator(indexInGroup: Int, sizeOfGroup: Int,
                         groupIndex: Int, groupSum: Int,
                         groupMaxValue: Int, indexOfMaxInGroup: Int, maxStartsGroup: Boolean,
                         moves: IndexedSeq[Move]
                         ) {
    def setNewGroupMaxValue(weight: Int) = copy(
      groupMaxValue = weight,
      indexOfMaxInGroup = indexInGroup,
      maxStartsGroup = (indexInGroup == 0)
    )
    def areAllWeightsIdenticalAndIsThereMoreThanOne: Boolean = {
      maxStartsGroup && (indexOfMaxInGroup == indexInGroup) && (sizeOfGroup > 1)
    }
    def canGroupBeCollapsed: Boolean = !areAllWeightsIdenticalAndIsThereMoreThanOne
    def monstersAfterMaxMonsterInGroup: Int = sizeOfGroup - indexOfMaxInGroup - 1
    def monstersBeforeMaxMonsterInGroup: Int = indexOfMaxInGroup
    def globalIndexOfMaxMonster: Int = groupIndex + indexOfMaxInGroup + 1
    def updateWithMovesToCollapseGroup(): Accumulator = {
      // Calculate the moves that collapse the group to a single value (assuming this is possible)...
      if (sizeOfGroup == 1) this  // With a single monster in the group, no eating will occur
      else {
        val newMoves = if (maxStartsGroup) {
          // Eat monsters to the right, then to the left:
          IndexedSeq.fill(monstersAfterMaxMonsterInGroup)(Move(globalIndexOfMaxMonster, 'R')) ++
          IndexedSeq.tabulate(monstersBeforeMaxMonsterInGroup)(i => Move(globalIndexOfMaxMonster - i, 'L' ))
        } else {
          // Eat monsters to the left, then the right:
          IndexedSeq.tabulate(monstersBeforeMaxMonsterInGroup) (i => Move(globalIndexOfMaxMonster - i, 'L' )) ++
          IndexedSeq.fill(monstersAfterMaxMonsterInGroup)(Move(groupIndex + 1, 'R'))
        }
        this.copy(moves = moves ++ newMoves)
      }
    }
    def startNextGroup(): Accumulator = {
      this.copy(indexInGroup = -1, sizeOfGroup = 0,
        groupIndex = groupIndex + 1, groupSum = 0, groupMaxValue = 0,
        indexOfMaxInGroup = -1, false)
    }
  }

  def solve(n: Int, k: Int, a: Array[Int], b: Array[Int]): Option[Seq[Move]] = {
    val initialAccumulator = Accumulator(-1, 0, 0, 0, 0, -1, false, IndexedSeq[Move]())
    val finalAcc = a.foldLeft[Option[Accumulator]](Some(initialAccumulator)) {
      (maybeAcc: Option[Accumulator], weight: Int) => {
        maybeAcc flatMap { acc =>
          if (b.length <= acc.groupIndex) None  // note enough weights in b to match all the weights in a
          else {
            val target = b(acc.groupIndex)

            // First update the fields that don't depend on other logic:
            var newAcc = acc.copy(
              groupSum = acc.groupSum + weight,
              indexInGroup = acc.indexInGroup + 1,
              sizeOfGroup = acc.sizeOfGroup + 1)

            // Track the maximum value in the group (that isn't surrounded by other biggest monsters),
            // as we can eat up the other monsters starting from it:
            if (newAcc.groupMaxValue < weight) newAcc = newAcc.setNewGroupMaxValue(weight)
            else if (newAcc.maxStartsGroup && (newAcc.groupMaxValue == weight)
              && (newAcc.indexOfMaxInGroup == newAcc.indexInGroup - 1)) {
              // If a block of max values (largest monsters) starts the group, then track the last one found,
              // as only the last one in the block can eat up the rest of the group (by eating right first, not left):
              newAcc = newAcc.copy(indexOfMaxInGroup = newAcc.indexInGroup)
            }

            if (newAcc.groupSum < target) Some(newAcc) // The group is incomplete, so keep iterating...
            else if (newAcc.groupSum > target) None // array a and b are incompatible, so no valid solution
            else if (!newAcc.canGroupBeCollapsed) None // target reached, but all monsters identical, so an impasse
            else {
              newAcc = newAcc.updateWithMovesToCollapseGroup()
              newAcc = newAcc.startNextGroup()
              Some(newAcc)
            }
          }
        }
      }
    }
    finalAcc match {
      case Some(acc) if acc.groupIndex == k => Some(acc.moves)  // Check that all b's were matched
      case _ => None
    }
  }
}
