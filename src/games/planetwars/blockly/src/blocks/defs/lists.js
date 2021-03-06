var Blockly = require('node-blockly/browser');

const LIST_COLOUR = 333;//Blockly.Blocks['lists'].HUE;

module.exports = {
    'filter': {
    init: function() {
      this.appendValueInput('LIST')
        .appendField('all')
        .appendField(new Blockly.FieldVariable('element'), 'ELEM_NAME')
        .appendField('in')
        .setCheck('List');
      this.appendValueInput('PREDICATE')
        .appendField('where')
        .setCheck('Boolean');
      this.setColour(LIST_COLOUR);
      this.setOutput(true, 'List');
      this.setTooltip('Returns the list fitered by the predicate in the `where` field.');
    }
  },
  'minmax': {
    init: function() {
      const modes = [['minimizes', 'MINIMIZE'],
                     ['maximizes', 'MAXIMIZE']];
      this.appendValueInput('LIST')
        .appendField(new Blockly.FieldVariable('element'), 'ELEM_NAME')
        .appendField('in')

        .setCheck('List');
      this.appendValueInput('EXPR')
        .appendField('that')
        .appendField(new Blockly.FieldDropdown(modes), 'MODE')
        .setCheck('Number');
      this.setColour(LIST_COLOUR);
      this.setTooltip('Returns the minimum or maximum of an expression when going trough a list.');
      this.setOutput(true);
    }
  },
  'forEach': {
    init: function() {
      this.setPreviousStatement(true);
      this.setNextStatement(true);
      this.appendValueInput('LIST')
        .appendField('for each')
        .appendField(new Blockly.FieldVariable('element'), 'ELEM_NAME')
        .appendField('in')
        .setCheck('List');
      this.appendStatementInput('DO')
        .appendField('do');
      this.setColour(LIST_COLOUR);
      this.setTooltip('Loops over every element in a list.');
    }
  }
};
