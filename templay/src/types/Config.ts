interface ConfigExternalEditor {
	command: string;
	args: string;
}

interface ConfigTemplate {
	name: string;
	content: string;
}

interface Config {
	version: number;
	external_editor: ConfigExternalEditor;
	templates: ConfigTemplate[];
}
